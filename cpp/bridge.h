#pragma once	// of course.
#include "../sbmod/surge/src/common/SurgeSynthesizer.h"

typedef SurgeSynthesizer::ID ID;

extern "C" {	// linkage?
	SurgeSynthesizer* create_engine(float sr);
	void destroy_engine(SurgeSynthesizer* surge);
	void destroy_parameter(Parameter* p);
	// note 1.
	int getNumInputs(SurgeSynthesizer* surge);
	int getNumOutputs(SurgeSynthesizer* surge);
	int getBlockSize(SurgeSynthesizer* surge);
	int getSynthSideId(const SurgeSynthesizer::ID* id);
#define CSUR const SurgeSynthesizer* surge
#define NSUR SurgeSynthesizer* surge
#define IAT1 const ID* index, char* text
#define IDPO const ID* index
	bool fromSynthSideId			(CSUR, int i, ID* q);
	ID idForParameter			(CSUR, const Parameter* p);
	void getParameterDisplay		(CSUR, IAT1);
	void getParameterDisplayAlt		(CSUR, IAT1);
	void getParameterName			(CSUR, IAT1);
	void getParameterNameExtendedByFXGroup	(CSUR, IAT1);
	void getParameterAccessibleName		(CSUR, IAT1);
	void getParameterMeta			(CSUR, IDPO, parametermeta* pm);
	float getParameter01			(CSUR, IDPO);
	bool setParameter01			(NSUR, IDPO, float value, bool external = false, bool force_integer = false);
	float normalizedToValue			(CSUR, IDPO, float val);
	float valueToNormalized			(CSUR, IDPO, float val);
	void sendParameterAutomation		(NSUR, IDPO, float val);
#undef CSUR
#undef NSUR
#undef IAT1
#undef IDPO
}

/*
 * *note 1:
 * i forgot to define these [three] functions before and it still worked.
 * apparently you don't need a header file for ffi (in this case).
 * will still keep it, though. no reason to remove it.
 */
