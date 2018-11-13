#pragma once
#include "Nuitrack.h"
#include <string>

using std::string;
using namespace tdv::nuitrack;

struct None {};

struct RustResult {
    enum {Ok, Err} tag;
    union {
        None empty;
        string error_msg;
    } value;

    static RustResult make_ok() {
        RustResult ret;
        ret.tag = Ok;
        ret.value.empty = None();
        return ret;
    }
    
    static RustResult make_err(string msg) {
        RustResult ret;
        ret.tag = Err;
        ret.value.error_msg = msg;;
        return ret;
    }
};

extern "C" RustResult nui_init();
