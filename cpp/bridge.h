#pragma once	// of course.
#include "../sbmod/surge/src/common/SurgeSynthesizer.h"

extern "C" {	// linkage?
	SurgeSynthesizer* create_engine(float sr);
	void destroy_engine(SurgeSynthesizer* surge);
}
// destroy?
