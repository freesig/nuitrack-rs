#include "helper.hpp"
#include <iostream>

using std::cout;
using std::endl;

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
        cout << "err" << endl;
        RHandTrackerWrapper ret;
        ret.result = result;
        return ret;
    } catch(...) {
        auto result = RustResult::make_unknown();
        cout << "err" << endl;
        RHandTrackerWrapper ret;
        ret.result = result;
        return ret;
    }
}
