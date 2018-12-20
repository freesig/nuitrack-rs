#include "helper.hpp"
#include <iostream>
#include <sstream>

using std::cout;
using std::endl;
using std::string;
using std::stringstream;
using namespace tdv::nuitrack;

static SmartPtr<SkeletonTracker> SKELETON_TRACKER;
static SmartPtr<DepthSensor> DEPTH_SENSOR;
static SmartPtr<ColorSensor> COLOR_SENSOR;

void create_skeleton_tracker() {
    std::lock_guard<std::mutex> lock(SKELETON_TRACKER.lock);
    if (SKELETON_TRACKER.ptr == nullptr) {
        SKELETON_TRACKER.ptr = SkeletonTracker::create();
    } 
}

void create_depth_sensor() {
    std::lock_guard<std::mutex> lock(DEPTH_SENSOR.lock);
    if (DEPTH_SENSOR.ptr == nullptr) {
        DEPTH_SENSOR.ptr = DepthSensor::create();
    } 
}

void create_color_sensor() {
    std::lock_guard<std::mutex> lock(COLOR_SENSOR.lock);
    if (COLOR_SENSOR.ptr == nullptr) {
        COLOR_SENSOR.ptr = ColorSensor::create();
    } 
}

//TODO these could be in a namespace to avoid nui_*
extern "C" RustResult nui_init(){
    string config_path = "";
    try {
        Nuitrack::init(config_path);
        return RustResult::make_ok();
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    }
}

extern "C" RustResult nui_set_rotation(int rotation){
    string r = std::to_string(rotation);
    try {
        Nuitrack::setConfigValue("DepthProvider.RotateAngle", r);
        return RustResult::make_ok();
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    }
}

extern "C" RustResult nui_run(){
    try {
        Nuitrack::run();
        return RustResult::make_ok();
    } catch (LicenseNotAcquiredException& e) {
        stringstream ss;
        ss << "LicenseNotAcquired exception: " << e.what() << endl;
        return RustResult::make_err(ss.str());
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    }
}

extern "C" RustResult nui_update(){
    try {
        if (SKELETON_TRACKER.ptr != nullptr) {
            Nuitrack::waitUpdate(SKELETON_TRACKER.ptr);
        } else if(DEPTH_SENSOR.ptr != nullptr) {
            Nuitrack::waitUpdate(DEPTH_SENSOR.ptr);
        } else if(COLOR_SENSOR.ptr != nullptr) {
            Nuitrack::waitUpdate(COLOR_SENSOR.ptr);
        }
        return RustResult::make_ok();
    } catch (LicenseNotAcquiredException& e) {
        stringstream ss;
        ss << "LicenseNotAcquired exception: " << e.what() << endl;
        return RustResult::make_err(ss.str());
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    }
}

extern "C" RustResult nui_release(){
    try {
        Nuitrack::release();
        return RustResult::make_ok();
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    }
}

std::vector<simple::Skeleton> to_simple(SkeletonData::Ptr sd) {
    const auto skeletons = sd->getSkeletons();
    std::vector<simple::Skeleton> s_skeletons;
    for(const auto & skeleton : skeletons) {
        auto s_skeleton = simple::Skeleton{
            .id = skeleton.id,
            .num_joints = skeleton.joints.size(),
            .joints = skeleton.joints.data()
        };
        s_skeletons.push_back(s_skeleton);
    }
    return s_skeletons;
}

simple::DepthFrame to_simple(DepthFrame::Ptr df) {
    auto ret = simple::DepthFrame{
        .rows = df->getRows(),
        .cols = df->getCols(),
        .id = df->getID(),
        .data = df->getData(),
        .time_stamp = df->getTimestamp()
    };
    return ret;
}

simple::RGBFrame to_simple(RGBFrame::Ptr cf) {
    auto ret = simple::RGBFrame{
        .rows = cf->getRows(),
        .cols = cf->getCols(),
        .id = cf->getID(),
        .data = cf->getData(),
        .time_stamp = cf->getTimestamp()
    };
    return ret;
}

extern "C" RustResult register_skeleton_closure(void (*cb)(void *, simple::SkeletonData), void * user_data) {
    try {
        create_skeleton_tracker();
        
        const auto wrapper = [=](const auto arg){ 
            auto s_skeletons = to_simple(arg);
            auto len = s_skeletons.size();
            auto sd = simple::SkeletonData{.skeletons = s_skeletons.data(), .len = len };
            cb(user_data, sd);
        };
        auto id = SKELETON_TRACKER.ptr->connectOnUpdate(wrapper);
        return RustResult::make_ok(id);
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    }
}

extern "C" RustResult register_depth_closure(void (*cb)(void *, simple::DepthFrame), void * user_data) {
    try {
        create_depth_sensor();
        
        const auto wrapper = [=](const auto arg){ 
            auto sd = to_simple(arg);
            cb(user_data, sd);
        };
        auto id = DEPTH_SENSOR.ptr->connectOnNewFrame(wrapper);
        return RustResult::make_ok(id);
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    }
}

extern "C" RustResult register_color_closure(void (*cb)(void *, simple::RGBFrame), void * user_data) {
    try {
        create_color_sensor();
        
        const auto wrapper = [=](const auto arg){ 
            auto sd = to_simple(arg);
            cb(user_data, sd);
        };
        auto id = COLOR_SENSOR.ptr->connectOnNewFrame(wrapper);
        return RustResult::make_ok(id);
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    }
}
