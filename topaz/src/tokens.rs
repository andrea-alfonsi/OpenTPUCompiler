#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Token<'src>
{
	Error,
	Whitespace(&'src str),
	Comment(&'src str),
	LineBreak,
	Word(&'src str),
	Number(&'src str),
	String(&'src str),
	ParenOpen,
	ParenClose,
	BracketOpen,
	BracketClose,
	BraceOpen,
	BraceClose,
	Dot,
	Comma,
	Colon,
	ColonColon,
	ArrowRight,
	ArrowLeft,
	HeavyArrowRight,
	Hash,
	Equal,
	Plus,
	Minus,
	Asterisk,
	Slash,
	Percent,
	Question,
	Exclamation,
	Ampersand,
	VerticalBar,
	Circumflex,
	Tilde,
	Grave,
	At,
	DoubleAmpersand,
	DoubleVerticalBar,
	DoubleEqual,
	ExclamationEqual,
	LessThan,
	DoubleLessThan,
  TripleLessThan,
	LessThanEqual,
	GreaterThan,
	DoubleGreaterThan,
	TripleGreaterThan,
	GreaterThanEqual
}