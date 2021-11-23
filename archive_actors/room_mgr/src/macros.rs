macro_rules! foo_info {
  ($target:expr,$actor:expr, $($arg:tt)*) => (
      logging::default().write_log(logging::WriteLogRequest{
        level: 3,
        body: format!($target,$($arg)*),
        actor: Some($actor),
      });
  );
}