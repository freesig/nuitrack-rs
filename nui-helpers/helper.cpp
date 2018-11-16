#include "helper.hpp"
#include <iostream>

using std::cout;
using std::endl;
using std::string;
using namespace tdv::nuitrack;

extern "C" RustResult nui_init(){
    string config_path = "";
    try {
        Nuitrack::init(config_path);
        return RustResult::make_ok();
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    }
}

extern "C" RHandTrackerWrapper create_hand_tracker() {
    try {
        auto ht = HandTracker::create();
        auto rht = RHandTracker{ht};
        auto result = RustResult::make_ok();
        auto ret = RHandTrackerWrapper{
            .result = result,
            .r_hand_tracker = rht,
        };
        return ret;
    } catch (const Exception& e) {
        auto result = RustResult::make_err(e.what());
        RHandTrackerWrapper ret;
        ret.result = result;
        return ret;
    } catch(...) {
        auto result = RustResult::make_unknown();
        RHandTrackerWrapper ret;
        ret.result = result;
        return ret;
    }
}

extern "C" RustResult hand_tracker_callback(HandTracker::Ptr hand_tracker, void (*hand_callback)(RHandTrackerDataPtr)) {
    try {
        auto callback_id = hand_tracker->connectOnUpdate(hand_callback);
        return RustResult::make_ok(callback_id);
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    } catch(...) {
        return RustResult::make_unknown();
    }
}

RHand to_rhand(Hand h){
    auto rh = RHand{
        .x = h.x,
        .y = h.y,
        .click = h.click,
        .pressure = h.pressure,
        .xReal = h.xReal,
        .yReal = h.yReal,
        .zReal = h.zReal
    };
    return rh;
}

extern "C" RustResult to_raw(RHandTrackerDataPtr ptr){
    try {
        auto hand_data = ptr->getUsersHands();
        auto n = hand_data.size();
        std::vector<RUserHands> ruh;
        for(auto & d : hand_data){
            auto r = RUserHands{ 
                .userId = d.userId,
                .leftHand = to_rhand(*d.leftHand),
                .rightHand = to_rhand(*d.rightHand)
            };
            ruh.push_back(r);
        }
        auto data = ruh.data();
        auto ret = RHandData{
            .data = data,
            .n = n,
        };
        return RustResult::make_ok(ret);
    } catch (const Exception& e) {
        return RustResult::make_err(e.what());
    } catch(...) {
        return RustResult::make_unknown();
    }
}
