// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use std::marker::PhantomData;

use iced::advanced::widget::{tree, Tree};
use iced::advanced::{layout, mouse, renderer, Clipboard, Layout, Shell, Widget};
use iced::widget::canvas;
use iced::widget::text::Shaping;
use iced::{mouse::Cursor, Element, Length, Rectangle, Size};

use crate::renderer::Renderer;

use super::Chart;

/// Chart container, turns [`Chart`]s to [`Widget`]s
pub struct ChartWidget<'a, Message, C>
where
    C: Chart<Message> + 'a,
{
    chart: C,
    width: Length,
    height: Length,
    shaping: Shaping,
    _marker: PhantomData<&'a Message>,
}

impl<'a, Message, C> ChartWidget<'a, Message, C>
where
    C: Chart<Message> + 'a,
{
    /// create a new [`ChartWidget`]
    pub fn new(chart: C) -> Self {
        Self {
            chart,
            width: Length::Fill,
            height: Length::Fill,
            shaping: Default::default(),
            _marker: Default::default(),
        }
    }

    /// set width
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// set height
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// set text shaping
    pub fn text_shaping(mut self, shaping: Shaping) -> Self {
        self.shaping = shaping;
        self
    }
}

impl<'a, Message, Theme, Renderer, C> Widget<Message, Theme, Renderer>
    for ChartWidget<'a, Message, C>
where
    C: Chart<Message>,
    Renderer: self::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn tag(&self) -> tree::Tag {
        struct Tag<T>(T);
        tree::Tag::of::<Tag<C::State>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(C::State::default())
    }

    #[inline]
    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.resolve(self.width, self.height, Size::ZERO);
        layout::Node::new(size)
    }

    #[inline]
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _defaults: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<C::State>();
        renderer.draw_chart(state, &self.chart, layout, self.shaping);
    }

    #[inline]
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let canvas_event = match event {
            iced::Event::Mouse(mouse_event) => Some(canvas::Event::Mouse(*mouse_event)),
            iced::Event::Keyboard(keyboard_event) => {
                Some(canvas::Event::Keyboard(keyboard_event.clone()))
            }
            _ => None,
        };
        if let Some(canvas_event) = canvas_event {
            let state = tree.state.downcast_mut::<C::State>();

            let (event_status, message) = self.chart.update(state, canvas_event, bounds, cursor);

            if let Some(message) = message {
                shell.publish(message);
            }

            if event_status == iced::event::Status::Captured {
                shell.capture_event();
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<C::State>();
        let bounds = layout.bounds();
        self.chart.mouse_interaction(state, bounds, cursor)
    }
}

impl<'a, Message, Theme, Renderer, C> From<ChartWidget<'a, Message, C>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    C: Chart<Message> + 'a,
    Renderer: self::Renderer,
{
    fn from(widget: ChartWidget<'a, Message, C>) -> Self {
        Element::new(widget)
    }
}
