#pragma once
#include <nuitrack/Nuitrack.h>
#include <string>
#include <sstream>
#include <iostream>
#include "simple.hpp"

template<class T>
struct SmartPtr {
    std::mutex lock;
    std::shared_ptr<T> ptr;
    void create();
};

struct ErrorMsg {
    std::string msg;
};

struct Nothing {};

// Has to match the const FooType: i32 = n; in Rust land error_conversion
enum Tag {Err = -1, 
    Ok = 0, 
    CallBackIdType = 1, 
    SkeletonDataType = 2};

union Value {
    Nothing empty;
    simple::SkeletonData skeleton_data;
    uint64_t callback_id;
    char error_msg[200];
};

struct RustResult {
    Tag tag;
    Value value;
    static RustResult make_ok() {
        RustResult ret = {.tag = Ok, .value = {.empty = Nothing()}};
        return ret;
    }
    
    static RustResult make_ok(uint64_t callback_id) {
        RustResult ret = {.tag = Ok, .value = {.callback_id = callback_id}};
        return ret;
    }
    
    static RustResult make_ok(simple::SkeletonData skeleton_data) {
        RustResult ret = {.tag = Ok, .value = {.skeleton_data = skeleton_data}};
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

extern "C" RustResult nui_init();
extern "C" RustResult nui_run();
extern "C" RustResult nui_release();
extern "C" RustResult register_closure(void (*cb)(void *, simple::SkeletonData), void *);
