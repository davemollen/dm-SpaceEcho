use crate::space_echo_parameters::SpaceEchoParameters;
use std::sync::Arc;
use vizia::{
  prelude::{
    Context, LayoutModifiers, StyleModifiers,
    Units::{Percentage, Pixels, Stretch}, LayoutType,
  },
  state::Model,
  views::{HStack, Label, VStack, ZStack}, modifiers::TextModifiers,
};
use vst::prelude::HostCallback;
mod param_knob;
pub use param_knob::ParamKnob;
mod param_checkbox;
pub use param_checkbox::ParamCheckbox;
mod param_radio_button;
pub use param_radio_button::ParamRadioButton;
mod ui_data;
pub use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./ui/style.css");

pub fn plugin_gui(cx: &mut Context, params: Arc<SpaceEchoParameters>, host: Option<HostCallback>) {
  cx.add_theme(STYLE);

  UiData {
    params: params.clone(),
    host,
  }
  .build(cx);
  
  HStack::new(cx, |cx| {
    HStack::new(cx, |cx| {
      ZStack::new(cx, |_| {})
        .col_index(1 as usize)
        .col_span(4 as usize)
        .border_radius_top_left(Pixels(8.0))
        .border_radius_top_right(Pixels(8.0))
        .background_color("#2d5f4f");

      ZStack::new(cx, |_| {})
        .col_index(1 as usize)
        .col_span(4 as usize)
        .row_index(1 as usize)
        .row_span(2 as usize)
        .border_color("#2d5f4f")
        .border_width(Pixels(2.0))
        .border_radius_bottom_left(Pixels(8.0))
        .border_radius_bottom_right(Pixels(8.0));

      ParamKnob::new(
        cx,
        UiData::params,
        &params.input,
        |params| &params.input,
        |val| ParamChangeEvent::SetInput(val),
      )
        .row_index(1 as usize)
        .col_index(0 as usize)
        .border_color("#2d5f4f")
        .border_width(Pixels(2.0));

      ParamCheckbox::new(
        cx,
        UiData::params,
        &params.time_link,
        |params| &params.time_link,
        |val| ParamChangeEvent::SetTimeLink(val),
      ).row_index(0 as usize).col_index(1 as usize);
      ParamKnob::new(
        cx,
        UiData::params,
        &params.time_left,
        |params| &params.time_left,
        |val| ParamChangeEvent::SetTimeLeft(val),
      ).row_index(0 as usize).col_index(2 as usize);
      ParamKnob::new(
        cx,
        UiData::params,
        &params.time_right,
        |params| &params.time_right,
        |val| ParamChangeEvent::SetTimeRight(val),
      ).row_index(0 as usize).col_index(3 as usize);
      ParamKnob::new(
        cx,
        UiData::params,
        &params.feedback,
        |params| &params.feedback,
        |val| ParamChangeEvent::SetFeedback(val),
      ).row_index(0 as usize).col_index(4 as usize);

      ParamCheckbox::new(
        cx,
        UiData::params,
        &params.hold,
        |params| &params.hold,
        |val| ParamChangeEvent::SetHold(val),
      ).row_index(1 as usize).col_index(3 as usize);
      ParamKnob::new(
        cx,
        UiData::params,
        &params.wow_and_flutter,
        |params| &params.wow_and_flutter,
        |val| ParamChangeEvent::SetWowAndFlutter(val),
      ).row_index(1 as usize).col_index(4 as usize);

      ParamRadioButton::new(
        cx,
        UiData::params,
        &params.time_mode,
        |params| &params.time_mode,
        |val| ParamChangeEvent::SetTimeMode(val)
      ).row_index(2 as usize).col_index(1 as usize).col_span(2 as usize);
      ParamRadioButton::new(
        cx,
        UiData::params,
        &params.channel_mode,
        |params| &params.channel_mode,
        |val| ParamChangeEvent::SetChannelMode(val)
      ).row_index(2 as usize).col_index(3 as usize).col_span(2 as usize);
    })
      .layout_type(LayoutType::Grid)
      .grid_cols(vec![Pixels(152.0); 5])
      .grid_rows(vec![Pixels(184.0); 3])
      .width(Pixels(76.0 * 5.0))
      .top(Pixels(60.0));

    HStack::new(cx, |cx| {
      ZStack::new(cx, |_| {})
        .col_index(0 as usize)
        .col_span(2 as usize)
        .border_radius_top_left(Pixels(8.0))
        .background_color("#2d5f4f");

      ZStack::new(cx, |_| {})
        .col_index(0 as usize)
        .col_span(2 as usize)
        .row_index(1 as usize)
        .row_span(2 as usize)
        .border_color("#2d5f4f")
        .border_width(Pixels(2.0))
        .border_radius_bottom_left(Pixels(8.0));

      ParamKnob::new(
        cx,
        UiData::params,
        &params.reverb,
        |params| &params.reverb,
        |val| ParamChangeEvent::SetReverb(val),
      ).row_index(0 as usize).col_index(0 as usize);
      ParamKnob::new(
        cx,
        UiData::params,
        &params.decay,
        |params| &params.decay,
        |val| ParamChangeEvent::SetDecay(val),
      ).row_index(0 as usize).col_index(1 as usize);
      
      ParamKnob::new(
        cx,
        UiData::params,
        &params.highpass_freq,
        |params| &params.highpass_freq,
        |val| ParamChangeEvent::SetHighpassFreq(val),
      ).row_index(1 as usize).col_index(0 as usize);
      ParamKnob::new(
        cx,
        UiData::params,
        &params.lowpass_freq,
        |params| &params.lowpass_freq,
        |val| ParamChangeEvent::SetLowpassFreq(val),
      ).row_index(1 as usize).col_index(1 as usize);

      ParamKnob::new(
        cx,
        UiData::params,
        &params.highpass_res,
        |params| &params.highpass_res,
        |val| ParamChangeEvent::SetHighpassRes(val),
      ).row_index(2 as usize).col_index(0 as usize);
      ParamKnob::new(
        cx,
        UiData::params,
        &params.lowpass_res,
        |params| &params.lowpass_res,
        |val| ParamChangeEvent::SetLowpassRes(val),
      ).row_index(2 as usize).col_index(1 as usize);
    })
      .layout_type(LayoutType::Grid)
      .grid_cols(vec![Pixels(152.0); 2])
      .grid_rows(vec![Pixels(184.0); 3])
      .width(Pixels(76.0 * 2.0))
      .top(Pixels(60.0))
      .left(Pixels(24.0));

    HStack::new(cx, |cx| {
      ParamKnob::new(
        cx,
        UiData::params,
        &params.output,
        |params| &params.output,
        |val| ParamChangeEvent::SetOutput(val),
      ).row_index(0 as usize).col_index(0 as usize);
      ParamKnob::new(
        cx,
        UiData::params,
        &params.mix,
        |params| &params.mix,
        |val| ParamChangeEvent::SetMix(val),
      ).row_index(0 as usize).col_index(1 as usize);

      ParamKnob::new(
        cx,
        UiData::params,
        &params.duck,
        |params| &params.duck,
        |val| ParamChangeEvent::SetDuck(val),
      ).row_index(1 as usize).col_index(0 as usize);
      
      ParamKnob::new(
        cx,
        UiData::params,
        &params.stereo,
        |params| &params.stereo,
        |val| ParamChangeEvent::SetStereo(val),
      ).row_index(1 as usize).col_index(1 as usize);

      ParamCheckbox::new(
        cx,
        UiData::params,
        &params.limiter,
        |params| &params.limiter,
        |val| ParamChangeEvent::SetLimiter(val),
      ).row_index(0 as usize).col_index(2 as usize);

      VStack::new(cx, |cx| {
        Label::new(cx, "DM")
          .font_size(32.0)
          .border_color("#2d5f4f")
          .border_radius(Pixels(24.0))
          .border_width(Pixels(4.0))
          .child_top(Pixels(2.0))
          .child_bottom(Pixels(2.0))
          .child_left(Pixels(12.0))
          .child_right(Pixels(12.0));
        Label::new(cx, "SpaceEcho")
          .font_size(22.0);
      })
        .row_index(2 as usize)
        .row_span(2 as usize)
        .col_index(0 as usize)
        .col_span(3 as usize)
        .row_between(Pixels(4.0))
        .child_space(Stretch(1.0))
        .right(Pixels(0.0));
    })
      .layout_type(LayoutType::Grid)
      .grid_cols(vec![Pixels(152.0); 3])
      .grid_rows(vec![Pixels(176.0); 4])
      .border_color("#2d5f4f")
      .border_width(Pixels(3.0))
      .border_radius_top_left(Pixels(8.0))
      .width(Percentage(110.0))
      .height(Percentage(110.0))
      .child_space(Pixels(8.0));
  })
  .background_color("#161616");
}
