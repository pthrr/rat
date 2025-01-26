use clack_plugin::{
    clack_export_entry,
    entry::{DefaultPluginFactory, SinglePluginEntry},
    host::{HostAudioProcessorHandle, HostMainThreadHandle, HostSharedHandle},
    plugin::{
        Plugin, PluginAudioProcessor, PluginDescriptor, PluginError, PluginMainThread, PluginShared,
    },
    prelude::PluginExtensions,
    process::{Audio, Events, PluginAudioConfiguration, Process, ProcessStatus},
};

pub struct CrabHowler;

impl Plugin for CrabHowler {
    type AudioProcessor<'a> = CrabHowlerAudioProcessor<'a>;
    type Shared<'a> = CrabHowlerShared;
    type MainThread<'a> = CrabHowlerMainThread<'a>;

    fn declare_extensions(builder: &mut PluginExtensions<Self>, shared: Option<&Self::Shared<'_>>) {
    }
}

impl DefaultPluginFactory for CrabHowler {
    fn get_descriptor() -> PluginDescriptor {
        use clack_plugin::plugin::features::*;

        PluginDescriptor::new("com.kwarf.crabhowler", "Crab Howler")
            .with_vendor("Kwarf")
            .with_features([INSTRUMENT, SYNTHESIZER, STEREO])
    }

    fn new_shared(host: HostSharedHandle) -> Result<Self::Shared<'_>, PluginError> {
        Ok(CrabHowlerShared {})
    }

    fn new_main_thread<'a>(
        host: HostMainThreadHandle<'a>,
        shared: &'a Self::Shared<'a>,
    ) -> Result<Self::MainThread<'a>, PluginError> {
        Ok(Self::MainThread { shared })
    }
}

pub struct CrabHowlerAudioProcessor<'a> {
    shared: &'a CrabHowlerShared,
}

impl<'a> PluginAudioProcessor<'a, CrabHowlerShared, CrabHowlerMainThread<'a>>
    for CrabHowlerAudioProcessor<'a>
{
    fn activate(
        host: HostAudioProcessorHandle<'a>,
        main_thread: &mut CrabHowlerMainThread<'a>,
        shared: &'a CrabHowlerShared,
        audio_config: PluginAudioConfiguration,
    ) -> Result<Self, PluginError> {
        todo!()
    }

    fn process(
        &mut self,
        process: Process,
        audio: Audio,
        events: Events,
    ) -> Result<ProcessStatus, PluginError> {
        todo!()
    }
}

pub struct CrabHowlerShared {}

impl<'a> PluginShared<'a> for CrabHowlerShared {}

pub struct CrabHowlerMainThread<'a> {
    shared: &'a CrabHowlerShared,
}

impl<'a> PluginMainThread<'a, CrabHowlerShared> for CrabHowlerMainThread<'a> {}

clack_export_entry!(SinglePluginEntry<CrabHowler>);
