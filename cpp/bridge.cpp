#include "bridge.h"

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
}
