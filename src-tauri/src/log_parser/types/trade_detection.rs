#[derive(Clone, Debug)]
pub struct TradeDetection {
  pub start: String,
  pub confirmation_line: String,
  pub failed_line: String,
  pub cancelled_line: String,
  pub receive_line_first_part: String,
  pub receive_line_second_part: String,
  pub platinum_name: String,
}

impl TradeDetection {
  pub fn new(
    start: String,
    confirmation_line: String,
    failed_line: String,
    cancelled_line: String,
    receive_line_first_part: String,
    receive_line_second_part: String,
    platinum_name: String,
  ) -> Self {
    TradeDetection {
      start,
      confirmation_line,
      failed_line,
      cancelled_line,
      receive_line_first_part,
      receive_line_second_part,
      platinum_name,
    }
  }
}