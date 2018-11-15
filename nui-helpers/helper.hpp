#pragma once
#include <nuitrack/Nuitrack.h>
#include <string>
#include <sstream>
#include <iostream>

using std::string;
using std::endl;
using namespace tdv::nuitrack;

struct ErrorMsg {
    string msg;
};

struct Nothing {};

enum Tag {Ok, Err};

union Value {
    Nothing empty;
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
    
    static RustResult make_err(string msg) {
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
    HandTracker::Ptr ptr = nullptr;
};

struct RHandTrackerWrapper {
    RustResult result;
    RHandTracker r_hand_tracker;
};

extern "C" RustResult nui_init();
extern "C" RHandTrackerWrapper create_hand_tracker();
