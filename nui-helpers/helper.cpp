#include "helper.hpp"

extern "C" RustResult nui_init(){
    string config_path = "";
    try {
        Nuitrack::init(config_path);
        return RustResult::make_ok();
    } catch (const Exception& g) {
        return RustResult::make_err(e.type());
    }
}

