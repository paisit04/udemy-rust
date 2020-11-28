#[cfg(test)]
mod tests
{
  extern crate phrases;
  #[test]
  fn english_greeting_hello()
  {
    assert_eq!("hello", phrases::greetings::english::hello());
  }
  
  #[test]
  #[should_panic]
  #[ignore]
  fn english_greeting_goodbye()
  {
    assert_eq!("goodbyee", phrases::greetings::english::goodbye());
  }
}
