#include "bridge.h"

typedef SurgeSynthesizer::ID ID;

class ErrCork : public SurgeSynthesizer::PluginLayer {
	public:
		void surgeParameterUpdated(const SurgeSynthesizer::ID &id, float d) override {}
    		void surgeMacroUpdated(long macroNum, float d) override {}
};

extern "C" {

	SurgeSynthesizer* create_engine(float sr) {
		auto* layer = new ErrCork();
		auto* surge = new SurgeSynthesizer(layer, "");

		surge->setSamplerate(sr);
		surge->time_data.tempo = 120;
		surge->time_data.ppqPos = 0;

		return surge;
	}

	void destroy_engine(SurgeSynthesizer* surge) {
		if (surge) delete surge;	// this just works?
	}

	// TODO: check if below and above even need the if.
	void destroy_parameter(Parameter* p) {
		if (p) delete p;
	}

	// header functions that don't get exported by bindgen.
	// could be hard-coded but i'd rather have it be a bit more verbose in exchange for correctness.
	int getNumInputs(SurgeSynthesizer* surge)	{ return surge->getNumInputs(); }
	int getNumOutputs(SurgeSynthesizer* surge)	{ return surge->getNumOutputs(); }
	int getBlockSize(SurgeSynthesizer* surge)	{ return surge->getBlockSize(); }
	// member functions that don't get exported by bindgen.
	int getSynthSideId(const ID* id)		{ return id->getSynthSideId(); }
	// more header functions. note 1.
#define CSUR const SurgeSynthesizer* surge
#define NSUR SurgeSynthesizer* surge
#define IAT1 const ID* index, char* text
#define IAT2 *index, text
#define IDPO const ID* index
	bool fromSynthSideId			(CSUR, int i, ID* q)		{ return surge->fromSynthSideId(i, *q); }
	ID idForParameter			(CSUR, const Parameter* p)	{ return surge->idForParameter(p); }
	void getParameterDisplay		(CSUR, IAT1)			{ return surge->getParameterDisplay(IAT2); }
	void getParameterDisplayAlt		(CSUR, IAT1)			{ return surge->getParameterDisplay(IAT2); }
	void getParameterName			(CSUR, IAT1)			{ return surge->getParameterName(IAT2); }
	void getParameterNameExtendedByFXGroup	(CSUR, IAT1)			{ return surge->getParameterNameExtendedByFXGroup(IAT2); }
	void getParameterAccessibleName		(CSUR, IAT1)			{ return surge->getParameterAccessibleName(IAT2); }
	void getParameterMeta			(CSUR, IDPO, parametermeta* pm)	{ return surge->getParameterMeta(*index, *pm); }
	float getParameter01			(CSUR, IDPO)			{ return surge->getParameter01(*index); }
	bool setParameter01			(NSUR, IDPO,
						float value,
						bool external,
						bool force_integer)
						{ return surge->setParameter01(
							*index, value, external, force_integer);
						} // this looks really bad.
	float normalizedToValue(CSUR, IDPO, float val)		{ return surge->normalizedToValue(*index, val); }
	float valueToNormalized(CSUR, IDPO, float val)		{ return surge->valueToNormalized(*index, val); }
	void sendParameterAutomation(NSUR, IDPO, float val)	{ return surge->sendParameterAutomation(*index, val); }
#undef CSUR
#undef NSUR
#undef IAT1
#undef IAT2
#undef IDPO
}
// EXTERNAL FALSE, FORCE_INTEGER FALSE.

/*
 * note 1:
 * remember c doesn't know what references are!
 * also, standalone functions no longer take const (as outer definition).
 * this is why the inner arguments are const. shows no change to what would be self.
 * it also prevents having to pass mutable references from rust.
 */
