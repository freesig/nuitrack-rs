#pragma once
#include <nuitrack/Nuitrack.h>
#include <string>
#include <sstream>
#include <iostream>


typedef std::shared_ptr<tdv::nuitrack::HandTrackerData> RHandTrackerDataPtr;

struct ErrorMsg {
    std::string msg;
};

struct Nothing {};

enum Tag {Ok, Err};

union Value {
    Nothing empty;
    uint64_t callback_id;
    void * hand_data;
    char error_msg[200];
};

struct RustResult {
    Tag tag;
    Value value;
    static RustResult make_ok() {
        RustResult ret = {.tag = Ok, .value = {.empty = Nothing()}};
        std::cout << "make ok" << std::endl;
        return ret;
    }
    
    static RustResult make_ok(uint64_t callback_id) {
        RustResult ret = {.tag = Ok, .value = {.callback_id = callback_id}};
        std::cout << "make ok" << std::endl;
        return ret;
    }
    
    static RustResult make_ok(void * hand_data) {
        RustResult ret = {.tag = Ok, .value = {.hand_data = hand_data}};
        std::cout << "make ok" << std::endl;
        return ret;
    }
    
    static RustResult make_err(std::string msg) {
        RustResult ret = {.tag = Err, .value = {.error_msg = {0}}};
        strcpy(ret.value.error_msg, msg.c_str());
        return ret;
    }
    
    static RustResult make_unknown() {
        RustResult ret = {.tag = Err, .value = {.error_msg = {0}}};
        strcpy(ret.value.error_msg, "Unknown Error");
        return ret;
    }
};

struct RHandTracker {
    tdv::nuitrack::HandTracker::Ptr ptr = nullptr;
};

struct RHandTrackerWrapper {
    RustResult result;
    RHandTracker r_hand_tracker;
};


extern "C" RustResult nui_init();
extern "C" RHandTrackerWrapper create_hand_tracker();
extern "C" RustResult hand_tracker_callback(tdv::nuitrack::HandTracker::Ptr, void (*hand_callback)(RHandTrackerDataPtr));
extern "C" RustResult to_raw(RHandTrackerDataPtr);
