pub mod greetings
{
  pub mod english;
  pub mod french
  {
    pub fn hello() -> String { "bonjour".to_string() }
    pub fn goodbye() -> String { "au revoir".to_string() }
  }
}

#[test]
fn english_greeting_hello()
{
  assert_eq!("hello", greetings::english::hello());
}

#[test]
#[should_panic]
#[ignore]
fn english_greeting_goodbye()
{
  assert_eq!("goodbyee", greetings::english::goodbye());
}