#pragma once
#include <nuitrack/Nuitrack.h>
#include <string>

using std::string;
using namespace tdv::nuitrack;

struct None {};

struct RustResult {
    enum {Ok, Err} tag;
    union {
        None empty;
        const char * error_msg;
    } value;

    static RustResult make_ok() {
        RustResult ret = {.tag = Ok, .value = {.empty = None()}};
        return ret;
    }
    
    static RustResult make_err(string msg) {
        RustResult ret = {.tag = Err, .value = {.error_msg = msg.c_str()}};
        return ret;
    }
};

extern "C" RustResult nui_init();
