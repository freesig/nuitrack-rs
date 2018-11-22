#include "helper.hpp"
#include <iostream>
#include <sstream>

using std::cout;
using std::endl;
using std::string;
using std::stringstream;
using namespace tdv::nuitrack;

static SmartPtr<SkeletonTracker> SKELETON_TRACKER;

void create_skeleton_tracker() {
    std::lock_guard<std::mutex> lock(SKELETON_TRACKER.lock);
    if (SKELETON_TRACKER.ptr == nullptr) {
        SKELETON_TRACKER.ptr = SkeletonTracker::create();
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

extern "C" RustResult nui_release(){
    try {
        Nuitrack::release();
        return RustResult::make_ok();
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    }
}

simple::SkeletonData to_simple(SkeletonData::Ptr sd) {
    const auto skeletons = sd->getSkeletons();
    std::vector<simple::Skeleton> s_skeletons;
    for(const auto & skeleton : skeletons) {
        auto s_skeleton = simple::Skeleton{
            .id = skeleton.id,
            .joints = skeleton.joints.data()
        };
        s_skeletons.push_back(s_skeleton);
    }
    auto ret = simple::SkeletonData{.skeletons = s_skeletons.data()};
    return ret;
}

extern "C" RustResult register_closure(void (*cb)(void *, simple::SkeletonData), void * user_data) {
    try {
        create_skeleton_tracker();

        const auto wrapper = [&](const auto arg){ 
            auto sd = to_simple(arg);
            cb(user_data, sd);
        };
        auto id = SKELETON_TRACKER.ptr->connectOnUpdate(wrapper);
        return RustResult::make_ok(id);
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    }
}
