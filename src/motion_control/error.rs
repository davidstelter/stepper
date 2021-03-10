/// An error that can occur while using [`SoftwareMotionControl`]
///
/// [`SoftwareMotionControl`]: super::SoftwareMotionControl
#[derive(Debug)]
pub enum Error<
    SetDirectionError,
    StepError,
    TimerError,
    NanosecondsToTicksError,
    DelayToTicksError,
> {
    /// Error while setting direction
    SetDirection(
        crate::SignalError<
            SetDirectionError,
            NanosecondsToTicksError,
            TimerError,
        >,
    ),

    /// Error while stepping the motor
    Step(crate::SignalError<StepError, NanosecondsToTicksError, TimerError>),

    /// Error while converting between time formats
    TimeConversion(
        TimeConversionError<NanosecondsToTicksError, DelayToTicksError>,
    ),

    /// Error while waiting for a step to finish
    StepDelay(TimerError),
}

/// An error occurred while converting between time formats
#[derive(Debug)]
pub enum TimeConversionError<NanosecondsToTicksError, DelayToTicksError> {
    /// Error converting from nanoseconds to timer ticks
    NanosecondsToTicks(NanosecondsToTicksError),

    /// Error converting from RampMaker delay value to timer ticks
    DelayToTicks(DelayToTicksError),
}

/// The software motion control was busy, or another generic error occurred
#[derive(Debug)]
pub enum BusyError<T> {
    /// The software motion control was busy
    ///
    /// This happens while a movement is going on, and the driver is not
    /// available.
    Busy,

    /// Another error has occurred
    Other(T),
}
