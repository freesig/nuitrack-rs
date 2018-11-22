#pragma once
#include <nuitrack/Nuitrack.h>
namespace simple {
    struct Skeleton{
        int id;
        const tdv::nuitrack::Joint * joints;
    };
    struct SkeletonData {
        Skeleton * skeletons; 
    };
};

