use crate::avm1::error::Error;
use crate::avm1::function::Executable;
use crate::avm1::globals::system::SystemCapabilities;
use crate::avm1::object::Object;
use crate::avm1::return_value::ReturnValue;
use crate::avm1::{Avm1, ScriptObject, TObject, Value};
use crate::context::UpdateContext;
use enumset::EnumSet;
use gc_arena::MutationContext;

macro_rules! capabilities_func {
    ($func_name: ident, $capability: expr) => {
        pub fn $func_name<'gc>(
            _avm: &mut Avm1<'gc>,
            context: &mut UpdateContext<'_, 'gc, '_>,
            _this: Object<'gc>,
            _args: &[Value<'gc>],
        ) -> Result<ReturnValue<'gc>, Error> {
            Ok(context.system.has_capability($capability).into())
        }
    };
}

macro_rules! inverse_capabilities_func {
    ($func_name: ident, $capability: expr) => {
        pub fn $func_name<'gc>(
            _avm: &mut Avm1<'gc>,
            context: &mut UpdateContext<'_, 'gc, '_>,
            _this: Object<'gc>,
            _args: &[Value<'gc>],
        ) -> Result<ReturnValue<'gc>, Error> {
            Ok((!context.system.has_capability($capability)).into())
        }
    };
}

macro_rules! capabilities_prop {
    ($gc_ctx: expr, $capabilities: expr, $($name:expr => $func:expr),* ) => {{
        $(
            $capabilities.add_property(
                $gc_ctx,
                $name,
                Executable::Native($func),
                None,
                EnumSet::empty()
            );
        )*
    }};
}

capabilities_func!(get_has_64_bit_support, SystemCapabilities::Process64Bit);
capabilities_func!(get_has_32_bit_support, SystemCapabilities::Process32Bit);
capabilities_func!(get_is_acrobat_embedded, SystemCapabilities::AcrobatEmbedded);
capabilities_func!(get_has_tls, SystemCapabilities::TLS);
capabilities_func!(get_has_accessibility, SystemCapabilities::Accessibility);
capabilities_func!(get_has_audio, SystemCapabilities::Audio);
capabilities_func!(get_has_audio_encoder, SystemCapabilities::AudioEncoder);
capabilities_func!(get_has_embedded_video, SystemCapabilities::EmbeddedVideo);

capabilities_func!(get_has_ime, SystemCapabilities::IME);
capabilities_func!(get_has_mp3, SystemCapabilities::MP3);
capabilities_func!(get_has_printing, SystemCapabilities::Printing);
capabilities_func!(
    get_has_screen_broadcast,
    SystemCapabilities::ScreenBroadcast
);
capabilities_func!(get_has_screen_playback, SystemCapabilities::ScreenPlayback);
capabilities_func!(get_has_streaming_audio, SystemCapabilities::StreamingAudio);
capabilities_func!(get_has_streaming_video, SystemCapabilities::StreamingVideo);
capabilities_func!(get_has_video_encoder, SystemCapabilities::VideoEncoder);
capabilities_func!(get_is_debugger, SystemCapabilities::Debugger);
inverse_capabilities_func!(
    get_is_local_file_read_disabled,
    SystemCapabilities::LocalFileRead
);
inverse_capabilities_func!(get_is_av_hardware_disabled, SystemCapabilities::AvHardware);
inverse_capabilities_func!(get_is_windowless_disabled, SystemCapabilities::WindowLess);

pub fn get_player_type<'gc>(
    _avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context.system.player_type.to_string().into())
}

pub fn get_screen_color<'gc>(
    _avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context.system.screen_color.to_string().into())
}

pub fn get_language<'gc>(
    avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context
        .system
        .language
        .get_language_code(avm.player_version)
        .into())
}

pub fn get_screen_resolution_x<'gc>(
    _avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context.system.screen_resolution.0.into())
}

pub fn get_screen_resolution_y<'gc>(
    _avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context.system.screen_resolution.1.into())
}

pub fn get_pixel_aspect_ratio<'gc>(
    _avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context.system.aspect_ratio.into())
}

pub fn get_screen_dpi<'gc>(
    _avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context.system.dpi.into())
}

pub fn get_manufacturer<'gc>(
    avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context
        .system
        .manufacturer
        .get_manufacturer_string(avm.player_version)
        .into())
}

pub fn get_os_name<'gc>(
    _avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context.system.os.to_string().into())
}

pub fn get_version<'gc>(
    avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context.system.get_version_string(avm).into())
}

pub fn get_server_string<'gc>(
    avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context.system.get_server_string(avm).into())
}

pub fn get_cpu_architecture<'gc>(
    _avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context.system.cpu_architecture.to_string().into())
}

pub fn get_max_idc_level<'gc>(
    _avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(context.system.idc_level.clone().into())
}

pub fn create<'gc>(
    gc_context: MutationContext<'gc, '_>,
    proto: Option<Object<'gc>>,
) -> Object<'gc> {
    let capabilities = ScriptObject::object(gc_context, proto);

    capabilities_prop!(gc_context, capabilities,
        "supports64BitProcesses" => get_has_64_bit_support,
        "supports32BitProcesses" => get_has_32_bit_support,
        "isEmbeddedInAcrobat" => get_is_acrobat_embedded,
        "hasTLS" => get_has_tls,
        "cpuArchitecture" => get_cpu_architecture,
        "hasAccessibility" => get_has_accessibility,
        "hasAudio" => get_has_audio,
        "hasAudioEncoder" => get_has_audio_encoder,
        "hasEmbeddedVideo" => get_has_embedded_video,
        "hasIME" => get_has_ime,
        "hasMP3" => get_has_mp3,
        "hasPrinting" => get_has_printing,
        "hasScreenBroadcast" => get_has_screen_broadcast,
        "hasScreenPlayback" => get_has_screen_playback,
        "hasStreamingAudio" => get_has_streaming_audio,
        "hasStreamingVideo" => get_has_streaming_video,
        "hasVideoEncoder" => get_has_video_encoder,
        "isDebugger" => get_is_debugger,
        "avHardwareDisable" => get_is_av_hardware_disabled,
        "localFileReadDisable" => get_is_local_file_read_disabled,
        "windowlessDisable" => get_is_windowless_disabled,
        "language" => get_language,
        "manufacturer" => get_manufacturer,
        "os" => get_os_name,
        "pixelAspectRatio" => get_pixel_aspect_ratio,
        "playerType"=>get_player_type,
        "screenColor" => get_screen_color,
        "screenDPI" => get_screen_dpi,
        "screenResolutionX" => get_screen_resolution_x,
        "screenResolutionY" => get_screen_resolution_y,
        "serverString" => get_server_string,
        "version" => get_version,
        "maxLevelIDC" => get_max_idc_level
    );

    capabilities.into()
}
