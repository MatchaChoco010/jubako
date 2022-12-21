// VNode Events list handled by jubako
#[macro_export(crate)]
macro_rules! event_list_macro {
    ($event_type:ident) => {
        $event_type! {
            abort, Abort => Event,
            animation_cancel, AnimationCancel => AnimationEvent,
            animation_end, AnimationEnd => AnimationEvent,
            animation_iteration, AnimationIteration => AnimationEvent,
            animation_start, AnimationStart => AnimationEvent,
            aux_click, AuxClick => MouseEvent,
            blur, Blur => FocusEvent,
            cancel, Cancel => Event,
            can_play, CanPlay => Event,
            can_play_through, CanPlayThrough => Event,
            change, Change => Event,
            click, Click => MouseEvent,
            close, Close => Event,
            context_menu, ContextMenu => MouseEvent,
            copy, Copy => Event,
            cue_change, CueChange => Event,
            cut, Cut => Event,
            double_click, DoubleClick => MouseEvent,
            drag, Drag => DragEvent,
            drag_end, DragEnd => DragEvent,
            drag_enter, DragEnter => DragEvent,
            // drag_exit, DragExit => DragEvent, // firefox only? use drag_leave instead
            drag_leave, DragLeave => DragEvent,
            drag_over, DragOver => DragEvent,
            drag_start, DragStart => DragEvent,
            drop, Drop => DragEvent,
            duration_change, DurationChange => Event,
            emptied, Emptied => Event,
            ended, Ended => Event,
            error, Error => Event,
            focus, Focus => FocusEvent,
            focus_in, FocusIn => FocusEvent,
            focus_out, FocusOut => FocusEvent,
            form_data, FormData => Event,
            got_pointer_capture, GotPointerCapture => PointerEvent,
            input, Input => InputEvent,
            invalid, Invalid => Event,
            key_down, KeyDown => KeyboardEvent,
            key_press, KeyPress => KeyboardEvent,
            key_up, KeyUp => KeyboardEvent,
            load, Load => Event,
            loaded_data, LoadedData => Event,
            loaded_metadata, LoadedMetadata => Event,
            load_start, LoadStart => ProgressEvent,
            // load_end, LoadEnd => ProgressEvent, // firefox only? use load instead
            lost_pointer_capture, LostPointerCapture => PointerEvent,
            mouse_down, MouseDown => MouseEvent,
            mouse_enter, MouseEnter => MouseEvent,
            mouse_leave, MouseLeave => MouseEvent,
            mouse_move, MouseMove => MouseEvent,
            mouse_out, MouseOut => MouseEvent,
            mouse_over, MouseOver => MouseEvent,
            mouse_up, MouseUp => MouseEvent,
            paste, Paste => Event,
            pause, Pause => Event,
            play, Play => Event,
            playing, Playing => Event,
            pointer_cancel, PointerCancel => PointerEvent,
            pointer_down, PointerDown => PointerEvent,
            pointer_enter, PointerEnter => PointerEvent,
            pointer_leave, PointerLeave => PointerEvent,
            pointer_move, PointerMove => PointerEvent,
            pointer_out, PointerOut => PointerEvent,
            pointer_over, PointerOver => PointerEvent,
            pointer_up, PointerUp => PointerEvent,
            progress, Progress => ProgressEvent,
            rate_change, RateChange => Event,
            reset, Reset => Event,
            resize, Resize => Event,
            scroll, Scroll => Event,
            security_policy_violation, SecurityPolicyViolation => Event,
            seeked, Seeked => Event,
            seeking, Seeking => Event,
            select, Select => Event,
            selection_change, SelectionChange => Event,
            select_start, SelectStart => Event,
            // show, Show => Event, // deprecated event
            slot_change, SlotChange => Event,
            stalled, Stalled => Event,
            submit, Submit => SubmitEvent,
            suspend, Suspend => Event,
            time_update, TimeUpdate => Event,
            toggle, Toggle => Event,
            touch_cancel, TouchCancel => TouchEvent,
            touch_end, TouchEnd => TouchEvent,
            touch_move, TouchMove => TouchEvent,
            touch_start, TouchStart => TouchEvent,
            transition_cancel, TransitionCancel => TransitionEvent,
            transition_end, TransitionEnd => TransitionEvent,
            transition_run, TransitionRun => TransitionEvent,
            transition_start, TransitionStart => TransitionEvent,
            volume_change, VolumeChange => Event,
            waiting, Waiting => Event,
            wheel, Wheel => WheelEvent,
        }
    };
}