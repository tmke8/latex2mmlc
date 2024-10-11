use crate::attribute::{FracAttr, MathVariant, OpAttr, ParenAttr, Stretchy, Style, TextTransform};
use crate::ops::{self, Op};
use crate::token::Token;

static COMMANDS: phf::Map<&'static str, Token> = phf::phf_map! {
    " " => Token::Space("1"),
    "!" => Token::Space("-0.1667"),
    "#" => Token::Letter('#'),
    "$" => Token::Letter('$'),
    "%" => Token::Letter('%'),
    "&" => Token::OpAmpersand,
    "," => Token::Space("0.1667"),
    ":" => Token::Space("0.2222"),
    ";" => Token::Space("0.2778"),
    "\\" => Token::NewLine,
    "_" => Token::Letter('_'),
    "AA" => Token::Letter('Å'), // TODO: valid in text mode only
    "AE" => Token::Letter('Æ'),
    "Alpha" => Token::UprightLetter('Α'),
    "And" => Token::OpAmpersand,
    "Bbbk" => Token::Letter('𝕜'),
    "Beta" => Token::UprightLetter('Β'),
    "Big" => Token::Big("1.623em"),
    "Bigg" => Token::Big("2.470em"),
    "Biggl" => Token::Big("2.470em"),
    "Biggr" => Token::Big("2.470em"),
    "Bigl" => Token::Big("1.623em"),
    "Bigr" => Token::Big("1.623em"),
    "Box" => Token::Letter('◻'),
    "Bumpeq" => Token::Operator(ops::GEOMETRICALLY_EQUIVALENT_TO),
    "Cap" => Token::Operator(ops::DOUBLE_INTERSECTION),
    "Chi" => Token::UprightLetter('Χ'),
    "Colon" => Token::Operator(ops::PROPORTION),
    "Cup" => Token::Operator(ops::DOUBLE_UNION),
    "DH" => Token::Letter('Ð'),
    "Dagger" => Token::Letter('‡'),
    "Delta" => Token::UprightLetter('Δ'),
    "Diamond" => Token::Letter('◊'),
    "Doteq" => Token::Operator(ops::GEOMETRICALLY_EQUAL_TO),
    "Downarrow" => Token::Paren(ops::DOWNWARDS_DOUBLE_ARROW, None, Stretchy::Inconsistent),
    "Epsilon" => Token::UprightLetter('Ε'),
    "Eta" => Token::UprightLetter('Η'),
    "Finv" => Token::Letter('Ⅎ'),
    "Game" => Token::Letter('⅁'),
    "Gamma" => Token::UprightLetter('Γ'),
    "Im" => Token::Letter('ℑ'),
    "Iota" => Token::UprightLetter('Ι'),
    "Join" => Token::Operator(ops::BOWTIE),
    "Kappa" => Token::UprightLetter('Κ'),
    "L" => Token::Letter('Ł'),
    "Lambda" => Token::UprightLetter('Λ'),
    "Leftarrow" => Token::Operator(ops::LEFTWARDS_DOUBLE_ARROW),
    "Leftrightarrow" => Token::Operator(ops::LEFT_RIGHT_DOUBLE_ARROW),
    "Lleftarrow" => Token::Operator(ops::LEFTWARDS_TRIPLE_ARROW),
    "Longleftarrow" => Token::Operator(ops::LONG_LEFTWARDS_DOUBLE_ARROW),
    "Longleftrightarrow" => Token::Operator(ops::LONG_LEFT_RIGHT_DOUBLE_ARROW),
    "Longrightarrow" => Token::Operator(ops::LONG_RIGHTWARDS_DOUBLE_ARROW),
    "Lsh" => Token::Operator(ops::UPWARDS_ARROW_WITH_TIP_LEFTWARDS),
    "Mu" => Token::UprightLetter('Μ'),
    "NG" => Token::Letter('Ŋ'),
    "Nu" => Token::UprightLetter('Ν'),
    "O" => Token::Letter('Ø'),
    "OE" => Token::Letter('Œ'),
    "Omega" => Token::UprightLetter('Ω'),
    "Omicron" => Token::UprightLetter('Ο'),
    "P" => Token::Letter('¶'),
    "Phi" => Token::UprightLetter('Φ'),
    "Pi" => Token::UprightLetter('Π'),
    "Pr" => Token::Function("Pr"),
    "Psi" => Token::UprightLetter('Ψ'),
    "Re" => Token::Letter('ℜ'),
    "Rho" => Token::UprightLetter('Ρ'),
    "Rightarrow" => Token::Operator(ops::RIGHTWARDS_DOUBLE_ARROW),
    "Rrightarrow" => Token::Operator(ops::RIGHTWARDS_TRIPLE_ARROW),
    "Rsh" => Token::Operator(ops::UPWARDS_ARROW_WITH_TIP_RIGHTWARDS),
    "S" => Token::Letter('§'),
    "Sigma" => Token::UprightLetter('Σ'),
    "Subset" => Token::Operator(ops::DOUBLE_SUBSET),
    "Supset" => Token::Operator(ops::DOUBLE_SUPERSET),
    "TH" => Token::Letter('Þ'),
    "Tau" => Token::UprightLetter('Τ'),
    "Theta" => Token::UprightLetter('Θ'),
    "Uparrow" => Token::Paren(ops::UPWARDS_DOUBLE_ARROW, None, Stretchy::Inconsistent),
    "Updownarrow" => Token::Paren(ops::UP_DOWN_DOUBLE_ARROW, None, Stretchy::Inconsistent),
    "Upsilon" => Token::UprightLetter('Υ'),
    "Vdash" => Token::Operator(ops::FORCES),
    "Vert" => Token::Paren(ops::DOUBLE_VERTICAL_LINE, None, Stretchy::PrePostfix),
    "Xi" => Token::UprightLetter('Ξ'),
    "Yleft" => Token::Operator(ops::LEFTWARDS_ARROW_TAIL),
    "Yright" => Token::Operator(ops::RIGHTWARDS_ARROW_TAIL),
    "Zeta" => Token::UprightLetter('Ζ'),
    "a" => Token::Letter('å'),
    "acute" => Token::OverUnder(ops::ACUTE_ACCENT, true, None),
    "ae" => Token::Letter('æ'),
    "aleph" => Token::Letter('ℵ'),
    "alpha" => Token::Letter('α'),
    "amalg" => Token::Operator(ops::AMALGAMATION_OR_COPRODUCT),
    "angle" => Token::Letter(ops::ANGLE),
    "approx" => Token::Operator(ops::ALMOST_EQUAL_TO),
    "approxeq" => Token::Operator(ops::ALMOST_EQUAL_OR_EQUAL_TO),
    "arccos" => Token::Function("arccos"),
    "arceq" => Token::Operator(ops::CORRESPONDS_TO), // from "stix"
    "arcsin" => Token::Function("arcsin"),
    "arctan" => Token::Function("arctan"),
    "arg" => Token::Function("arg"),
    "ascnode" => Token::Letter('☊'),
    "ast" => Token::Operator(ops::ASTERISK_OPERATOR),
    "astrosun" => Token::Letter('☉'),
    "asymp" => Token::Operator(ops::EQUIVALENT_TO),
    "awint" => Token::Operator(ops::ANTICLOCKWISE_INTEGRATION),
    "backepsilon" => Token::Operator(ops::SMALL_CONTAINS_AS_MEMBER),
    "backprime" => Token::Operator(ops::REVERSED_PRIME),
    "backsim" => Token::Operator(ops::REVERSED_TILDE),
    "backsimeq" => Token::Operator(ops::REVERSED_TILDE_EQUALS),
    "backslash" => Token::Paren(ops::REVERSE_SOLIDUS, Some(ParenAttr::Ordinary), Stretchy::Never),
    "bar" => Token::OverUnder(ops::MACRON, true, Some(OpAttr::StretchyFalse)),
    "barwedge" => Token::Operator(ops::NAND),
    "because" => Token::Operator(ops::BECAUSE),
    "begin" => Token::Begin,
    "beta" => Token::Letter('β'),
    "beth" => Token::Letter('ℶ'),
    "big" => Token::Big("1.2em"),
    "bigcap" => Token::BigOp(ops::N_ARY_INTERSECTION),
    "bigcirc" => Token::Operator(ops::LARGE_CIRCLE),
    "bigcup" => Token::BigOp(ops::N_ARY_UNION),
    "bigcupdot" => Token::BigOp(ops::N_ARY_UNION_OPERATOR_WITH_DOT),
    "bigg" => Token::Big("2.047em"),
    "biggl" => Token::Big("2.047em"),
    "biggr" => Token::Big("2.047em"),
    "bigl" => Token::Big("1.2em"),
    "bigodot" => Token::BigOp(ops::N_ARY_CIRCLED_DOT_OPERATOR),
    "bigoplus" => Token::BigOp(ops::N_ARY_CIRCLED_PLUS_OPERATOR),
    "bigotimes" => Token::BigOp(ops::N_ARY_CIRCLED_TIMES_OPERATOR),
    "bigr" => Token::Big("1.2em"),
    "bigsqcap" => Token::BigOp(ops::N_ARY_SQUARE_INTERSECTION_OPERATOR),
    "bigsqcup" => Token::BigOp(ops::N_ARY_SQUARE_UNION_OPERATOR),
    "bigstar" => Token::Letter(ops::BLACK_STAR),
    "bigtimes" => Token::BigOp(ops::N_ARY_TIMES_OPERATOR),
    "bigtriangledown" => Token::Operator(ops::WHITE_DOWN_POINTING_TRIANGLE),
    "bigtriangleup" => Token::Letter('△'),
    "biguplus" => Token::BigOp(ops::N_ARY_UNION_OPERATOR_WITH_PLUS),
    "bigvee" => Token::BigOp(ops::N_ARY_LOGICAL_OR),
    "bigwedge" => Token::BigOp(ops::N_ARY_LOGICAL_AND),
    "binom" => Token::Binom(None),
    "blacklozenge" => Token::Letter(ops::BLACK_LOZENGE),
    "blacksquare" => Token::Letter(ops::BLACK_SQUARE),
    "bm" => Token::Transform(Some(TextTransform::BoldItalic), None),
    "boldsymbol" => Token::Transform(Some(TextTransform::BoldItalic), None),
    "bot" => Token::Operator(ops::UP_TACK),
    "botdoteq" => Token::Operator(ops::EQUALS_SIGN_WITH_DOT_BELOW),
    "boxbox" => Token::Operator(ops::SQUARED_SQUARE),
    "boxbslash" => Token::Operator(ops::SQUARED_FALLING_DIAGONAL_SLASH),
    "boxdot" => Token::Operator(ops::SQUARED_DOT_OPERATOR),
    "boxminus" => Token::Operator(ops::SQUARED_MINUS),
    "boxplus" => Token::Operator(ops::SQUARED_PLUS),
    "boxslash" => Token::Operator(ops::SQUARED_RISING_DIAGONAL_SLASH),
    "boxtimes" => Token::Operator(ops::SQUARED_TIMES),
    "breve" => Token::OverUnder(ops::BREVE, true, None),
    "bullet" => Token::Operator(ops::BULLET_OPERATOR),
    "bumpeq" => Token::Operator(ops::DIFFERENCE_BETWEEN),
    "cap" => Token::Operator(ops::INTERSECTION),
    "cdot" => Token::Operator(ops::MIDDLE_DOT),
    "cdots" => Token::Operator(ops::MIDLINE_HORIZONTAL_ELLIPSIS),
    "centerdot" => Token::Operator(ops::BULLET_OPERATOR),
    "cfrac" => Token::Frac(Some(FracAttr::CFracStyle)),
    "check" => Token::OverUnder(ops::CARON, true, Some(OpAttr::StretchyFalse)),
    "checkmark" => Token::Letter('✓'),
    "chi" => Token::Letter('χ'),
    "circ" => Token::Operator(ops::RING_OPERATOR),
    "circeq" => Token::Operator(ops::RING_EQUAL_TO),
    "circlearrowleft" => Token::Operator(ops::ANTICLOCKWISE_OPEN_CIRCLE_ARROW),
    "circlearrowright" => Token::Operator(ops::CLOCKWISE_OPEN_CIRCLE_ARROW),
    "circledR" => Token::Letter(ops::CIRCLED_LATIN_CAPITAL_LETTER_R),
    "circledS" => Token::Letter(ops::CIRCLED_LATIN_CAPITAL_LETTER_S),
    "circledast" => Token::Operator(ops::CIRCLED_ASTERISK_OPERATOR),
    "circledcirc" => Token::Operator(ops::CIRCLED_RING_OPERATOR),
    "circleddash" => Token::Operator(ops::CIRCLED_DASH),
    "cirfnint" => Token::Operator(ops::CIRCULATION_FUNCTION),
    "clubsuit" => Token::Letter('♣'),
    "colon" => Token::Letter(':'),
    "coloneq" => Token::Operator(ops::COLON_EQUALS),
    "complement" => Token::Letter(ops::COMPLEMENT),
    "cong" => Token::Operator(ops::APPROXIMATELY_EQUAL_TO),
    "coprod" => Token::BigOp(ops::N_ARY_COPRODUCT),
    "copyright" => Token::Letter('©'),
    "cos" => Token::Function("cos"),
    "cosh" => Token::Function("cosh"),
    "cot" => Token::Function("cot"),
    "coth" => Token::Function("coth"),
    "csc" => Token::Function("csc"),
    "cup" => Token::Operator(ops::UNION),
    "curlyeqprec" => Token::Operator(ops::EQUAL_TO_OR_PRECEDES),
    "curlyeqsucc" => Token::Operator(ops::EQUAL_TO_OR_SUCCEEDS),
    "curlyvee" => Token::Operator(ops::CURLY_LOGICAL_OR),
    "curlywedge" => Token::Operator(ops::CURLY_LOGICAL_AND),
    "curvearrowleft" => Token::Operator(ops::ANTICLOCKWISE_TOP_SEMICIRCLE_ARROW),
    "curvearrowright" => Token::Operator(ops::CLOCKWISE_TOP_SEMICIRCLE_ARROW),
    "dag" => Token::Letter('†'),
    "dagger" => Token::Letter('†'),
    "daleth" => Token::Letter('ℸ'),
    "dashcolon" => Token::Operator(ops::EXCESS),
    "dashv" => Token::Operator(ops::LEFT_TACK),
    "dbinom" => Token::Binom(Some(FracAttr::DisplayStyleTrue)),
    "ddag" => Token::Letter('‡'),
    "ddagger" => Token::Letter('‡'),
    "ddot" => Token::OverUnder(ops::DIAERESIS, true, None),
    "ddots" => Token::Operator(ops::DOWN_RIGHT_DIAGONAL_ELLIPSIS),
    "deg" => Token::Function("deg"),
    "delta" => Token::Letter('δ'),
    "det" => Token::Function("det"),
    "dfrac" => Token::Frac(Some(FracAttr::DisplayStyleTrue)),
    "dh" => Token::Letter('ð'),
    "diamond" => Token::Operator(ops::DIAMOND_OPERATOR),
    "diamondsuit" => Token::Letter('♢'),
    "digamma" => Token::Letter('ϝ'),
    "dim" => Token::Function("dim"),
    "displaystyle" => Token::Style(Style::DisplayStyle),
    "div" => Token::Operator(ops::DIVISION_SIGN),
    "divideontimes" => Token::Operator(ops::DIVISION_TIMES),
    "dj" => Token::Letter('đ'),
    "dot" => Token::OverUnder(ops::DOT_ABOVE, true, None),
    "doteq" => Token::Operator(ops::APPROACHES_THE_LIMIT),
    "doteqdot" => Token::Operator(ops::GEOMETRICALLY_EQUAL_TO),
    "dotplus" => Token::Operator(ops::DOT_PLUS),
    "dots" => Token::Operator(ops::HORIZONTAL_ELLIPSIS),
    "dotsminusdots" => Token::Operator(ops::GEOMETRIC_PROPORTION),
    "downarrow" => Token::Paren(ops::DOWNWARDS_ARROW, None, Stretchy::Inconsistent),
    "downdownarrows" => Token::Operator(ops::DOWNWARDS_PAIRED_ARROWS),
    "downharpoonleft" => Token::Operator(ops::DOWNWARDS_HARPOON_WITH_BARB_LEFTWARDS),
    "downharpoonright" => Token::Operator(ops::DOWNWARDS_HARPOON_WITH_BARB_RIGHTWARDS),
    "dprime" => Token::Operator(ops::DOUBLE_PRIME),
    "earth" => Token::Letter('♁'),
    "ell" => Token::Letter('ℓ'),
    "empty" => Token::Letter(ops::EMPTY_SET),
    "emptyset" => Token::Letter(ops::EMPTY_SET),
    "end" => Token::End,
    "epsilon" => Token::Letter('ϵ'),
    "eqcirc" => Token::Operator(ops::RING_IN_EQUAL_TO),
    "eqcolon" => Token::Operator(ops::EQUALS_COLON),
    "eqdef" => Token::Operator(ops::EQUAL_TO_BY_DEFINITION), // from "stix"
    "eqsim" => Token::Operator(ops::MINUS_TILDE),
    "eqslantgtr" => Token::Operator(ops::SLANTED_EQUAL_TO_OR_GREATER_THAN),
    "eqslantless" => Token::Operator(ops::SLANTED_EQUAL_TO_OR_LESS_THAN),
    "equiv" => Token::Operator(ops::IDENTICAL_TO),
    "erf" => Token::Function("erf"),
    "erfc" => Token::Function("erfc"),
    "eta" => Token::Letter('η'),
    "eth" => Token::Letter('ð'),
    "euro" => Token::Letter('€'),
    "exists" => Token::Operator(ops::THERE_EXISTS),
    "exp" => Token::Function("exp"),
    "fallingdotseq" => Token::Operator(ops::APPROXIMATELY_EQUAL_TO_OR_THE_IMAGE_OF),
    "fint" => Token::Operator(ops::INTEGRAL_AVERAGE_WITH_SLASH),
    "flat" => Token::Letter('♭'),
    "forall" => Token::Operator(ops::FOR_ALL),
    "frac" => Token::Frac(None),
    "frown" => Token::Operator(ops::FROWN),
    "gamma" => Token::Letter('γ'),
    "gcd" => Token::Function("gcd"),
    "ge" => Token::Operator(ops::GREATER_THAN_OR_EQUAL_TO),
    "genfrac" => Token::Genfrac,
    "geq" => Token::Operator(ops::GREATER_THAN_OR_EQUAL_TO),
    "geqq" => Token::Operator(ops::GREATER_THAN_OVER_EQUAL_TO),
    "geqslant" => Token::Operator(ops::GREATER_THAN_OR_SLANTED_EQUAL_TO),
    "gets" => Token::Operator(ops::LEFTWARDS_ARROW),
    "gg" => Token::Operator(ops::MUCH_GREATER_THAN),
    "gimel" => Token::Letter('ℷ'),
    "gneq" => Token::Operator(ops::GREATER_THAN_AND_SINGLE_LINE_NOT_EQUAL_TO),
    "gneqq" => Token::Operator(ops::GREATER_THAN_BUT_NOT_EQUAL_TO),
    "grave" => Token::OverUnder(ops::GRAVE_ACCENT, true, None),
    "gt" => Token::OpGreaterThan,
    "gtrapprox" => Token::Operator(ops::GREATER_THAN_OR_APPROXIMATE),
    "gtrless" => Token::Operator(ops::GREATER_THAN_OR_LESS_THAN),
    "gtrsim" => Token::Operator(ops::GREATER_THAN_OR_EQUIVALENT_TO),
    "hat" => Token::OverUnder(ops::CIRCUMFLEX_ACCENT, true, Some(OpAttr::StretchyFalse)),
    "hbar" => Token::Letter('ℏ'),
    "heartsuit" => Token::Letter('♡'),
    "hom" => Token::Function("hom"),
    "hookleftarrow" => Token::Operator(ops::LEFTWARDS_ARROW_WITH_HOOK),
    "hookrightarrow" => Token::Operator(ops::RIGHTWARDS_ARROW_WITH_HOOK),
    "hslash" => Token::Letter('ℏ'),
    "iff" => Token::Operator(ops::LONG_LEFT_RIGHT_DOUBLE_ARROW),
    "iiiint" => Token::Integral(ops::QUADRUPLE_INTEGRAL_OPERATOR),
    "iiint" => Token::Integral(ops::TRIPLE_INTEGRAL),
    "iint" => Token::Integral(ops::DOUBLE_INTEGRAL),
    "imath" => Token::Letter('ı'),
    "impliedby" => Token::Operator(ops::LONG_LEFTWARDS_DOUBLE_ARROW),
    "implies" => Token::Operator(ops::LONG_RIGHTWARDS_DOUBLE_ARROW),
    "in" => Token::Operator(ops::ELEMENT_OF),
    "inf" => Token::Lim("inf"),
    "infty" => Token::Letter(ops::INFINITY),
    "int" => Token::Integral(ops::INTEGRAL),
    "intBar" => Token::Operator(ops::INTEGRAL_WITH_DOUBLE_STROKE),
    "intbar" => Token::Operator(ops::FINITE_PARTL_INTEGRAL),
    "intclockwise" => Token::Operator(ops::CLOCKWISE_INTEGRAL),
    "intercal" => Token::Operator(ops::INTERCALATE),
    "iota" => Token::Letter('ι'),
    "jmath" => Token::Letter('ȷ'),
    "jupiter" => Token::Letter('♃'),
    "kappa" => Token::Letter('κ'),
    "ker" => Token::Function("ker"),
    "kernelcontraction" => Token::Operator(ops::HOMOTHETIC),
    "l" => Token::Letter('ł'),
    "lVert" => Token::Paren(ops::DOUBLE_VERTICAL_LINE, None, Stretchy::PrePostfix),
    "lambda" => Token::Letter('λ'),
    "land" => Token::Operator(ops::LOGICAL_AND),
    "langle" => Token::Paren(ops::MATHEMATICAL_LEFT_ANGLE_BRACKET, None, Stretchy::Always),
    "lbrace" => Token::Paren(ops::LEFT_CURLY_BRACKET, None, Stretchy::Always),
    "lbrack" => Token::Paren(ops::LEFT_SQUARE_BRACKET, None, Stretchy::Always),
    "lceil" => Token::Paren(ops::LEFT_CEILING, None, Stretchy::Always),
    "ldots" => Token::Operator(ops::HORIZONTAL_ELLIPSIS),
    "le" => Token::Operator(ops::LESS_THAN_OR_EQUAL_TO),
    "left" => Token::Left,
    "leftarrow" => Token::Operator(ops::LEFTWARDS_ARROW),
    "leftarrowtail" => Token::Operator(ops::LEFTWARDS_ARROW_WITH_TAIL),
    "leftharpoondown" => Token::Operator(ops::LEFTWARDS_HARPOON_WITH_BARB_DOWNWARDS),
    "leftharpoonup" => Token::Operator(ops::LEFTWARDS_HARPOON_WITH_BARB_UPWARDS),
    "leftleftarrows" => Token::Operator(ops::LEFTWARDS_PAIRED_ARROWS),
    "leftrightarrow" => Token::Operator(ops::LEFT_RIGHT_ARROW),
    "leftrightarrows" => Token::Operator(ops::LEFTWARDS_ARROW_OVER_RIGHTWARDS_ARROW),
    "leftrightharpoons" => Token::Operator(ops::LEFTWARDS_HARPOON_OVER_RIGHTWARDS_HARPOON),
    "leftrightsquigarrow" => Token::Operator(ops::LEFT_RIGHT_WAVE_ARROW),
    "leftthreetimes" => Token::Operator(ops::LEFT_SEMIDIRECT_PRODUCT),
    "leq" => Token::Operator(ops::LESS_THAN_OR_EQUAL_TO),
    "leqq" => Token::Operator(ops::LESS_THAN_OVER_EQUAL_TO),
    "leqslant" => Token::Operator(ops::LESS_THAN_OR_SLANTED_EQUAL_TO),
    "lessapprox" => Token::Operator(ops::LESS_THAN_OR_APPROXIMATE),
    "lessdot" => Token::Operator(ops::LESS_THAN_WITH_DOT),
    "lesseqgtr" => Token::Operator(ops::LESS_THAN_EQUAL_TO_OR_GREATER_THAN),
    "lesseqqgtr" => Token::Operator(ops::LESS_THAN_ABOVE_DOUBLE_LINE_EQUAL_ABOVE_GREATER_THAN),
    "lessgtr" => Token::Operator(ops::LESS_THAN_OR_GREATER_THAN),
    "lesssim" => Token::Operator(ops::LESS_THAN_OR_EQUIVALENT_TO),
    "lfloor" => Token::Paren(ops::LEFT_FLOOR, None, Stretchy::Always),
    "lg" => Token::Function("lg"),
    "lgroup" => Token::Paren(ops::MATHEMATICAL_LEFT_FLATTENED_PARENTHESIS, None, Stretchy::Always),
    "lhd" => Token::Operator(ops::NORMAL_SUBGROUP_OF),
    "lightning" => Token::Operator(ops::DOWNWARDS_ZIGZAG_ARROW),
    "lim" => Token::Lim("lim"),
    "liminf" => Token::Lim("lim inf"),
    "limits" => Token::Limits,
    "limsup" => Token::Lim("lim sup"),
    "ll" => Token::Operator(ops::MUCH_LESS_THAN),
    "llbracket" => Token::Paren(ops::MATHEMATICAL_LEFT_WHITE_SQUARE_BRACKET, None, Stretchy::Always),
    "llcorner" => Token::Letter(ops::BOTTOM_LEFT_CORNER),
    "lll" => Token::Operator(ops::VERY_MUCH_LESS_THAN),
    "ln" => Token::Function("ln"),
    "lneq" => Token::Operator(ops::LESS_THAN_AND_SINGLE_LINE_NOT_EQUAL_TO),
    "lneqq" => Token::Operator(ops::LESS_THAN_BUT_NOT_EQUAL_TO),
    "lnot" => Token::Operator(ops::NOT_SIGN),
    "log" => Token::Function("log"),
    "longleftarrow" => Token::Operator(ops::LONG_LEFTWARDS_ARROW),
    "longleftrightarrow" => Token::Operator(ops::LONG_LEFT_RIGHT_ARROW),
    "longmapsto" => Token::Operator(ops::LONG_RIGHTWARDS_ARROW_FROM_BAR),
    "longrightarrow" => Token::Operator(ops::LONG_RIGHTWARDS_ARROW),
    "looparrowleft" => Token::Operator(ops::LEFTWARDS_ARROW_WITH_LOOP),
    "looparrowright" => Token::Operator(ops::RIGHTWARDS_ARROW_WITH_LOOP),
    "lor" => Token::Operator(ops::LOGICAL_OR),
    "lozenge" => Token::Letter('◊'),
    "lrcorner" => Token::Letter(ops::BOTTOM_RIGHT_CORNER),
    "lt" => Token::OpLessThan,
    "ltimes" => Token::Operator(ops::LEFT_NORMAL_FACTOR_SEMIDIRECT_PRODUCT),
    "lvert" => Token::Paren(ops::VERTICAL_LINE, None, Stretchy::PrePostfix),
    "maltese" => Token::Letter('✠'),
    "mapsto" => Token::Operator(ops::RIGHTWARDS_ARROW_FROM_BAR),
    "mars" => Token::Letter('♂'),
    "mathbb" => Token::Transform(Some(TextTransform::DoubleStruck), None),
    "mathbf" => Token::Transform(Some(TextTransform::Bold), None),
    "mathcal" => Token::Transform(Some(TextTransform::Script), None),
    "mathfrak" => Token::Transform(Some(TextTransform::Fraktur), None),
    "mathit" => Token::Transform(Some(TextTransform::Italic), None),
    "mathrm" => Token::Transform(None, Some(MathVariant::Normal)),
    "mathscr" => Token::Transform(Some(TextTransform::Script), None),
    "mathsf" => Token::Transform(Some(TextTransform::SansSerif), None),
    "mathstrut" => Token::Mathstrut,
    "mathtt" => Token::Transform(Some(TextTransform::Monospace), None),
    "max" => Token::Lim("max"),
    "measeq" => Token::Operator(ops::MEASURED_BY), // from "stix"
    "measuredangle" => Token::Letter(ops::MEASURED_ANGLE),
    "mercury" => Token::Letter('☿'),
    "mho" => Token::Letter('℧'),
    "mid" => Token::Operator(ops::DIVIDES),
    "middle" => Token::Middle,
    "min" => Token::Lim("min"),
    "models" => Token::Operator(ops::TRUE),
    "mp" => Token::Operator(ops::MINUS_OR_PLUS_SIGN),
    "mu" => Token::Letter('μ'),
    "multimap" => Token::Operator(ops::MULTIMAP),
    "nLeftarrow" => Token::Operator(ops::LEFTWARDS_DOUBLE_ARROW_WITH_STROKE),
    "nLeftrightarrow" => Token::Operator(ops::LEFT_RIGHT_DOUBLE_ARROW_WITH_STROKE),
    "nRightarrow" => Token::Operator(ops::RIGHTWARDS_DOUBLE_ARROW_WITH_STROKE),
    "nabla" => Token::UprightLetter(ops::NABLA),
    "natural" => Token::Letter('♮'),
    "ne" => Token::Operator(ops::NOT_EQUAL_TO),
    "nearrow" => Token::Operator(ops::NORTH_EAST_ARROW),
    "neg" => Token::Operator(ops::NOT_SIGN),
    "neptune" => Token::Letter('♆'),
    "neq" => Token::Operator(ops::NOT_EQUAL_TO),
    "nequiv" => Token::Operator(ops::NOT_IDENTICAL_TO),
    "nexists" => Token::Operator(ops::THERE_DOES_NOT_EXIST),
    "ng" => Token::Letter('ŋ'),
    "ngeq" => Token::Operator(ops::NEITHER_GREATER_THAN_NOR_EQUAL_TO),
    "ngtr" => Token::Operator(ops::NOT_GREATER_THAN),
    "ngtrless" => Token::Operator(ops::NEITHER_GREATER_THAN_NOR_LESS_THAN),
    "ngtrsim" => Token::Operator(ops::NEITHER_GREATER_THAN_NOR_EQUIVALENT_TO),
    "ni" => Token::Operator(ops::CONTAINS_AS_MEMBER),
    "nleftarrow" => Token::Operator(ops::LEFTWARDS_ARROW_WITH_STROKE),
    "nleftrightarrow" => Token::Operator(ops::LEFT_RIGHT_ARROW_WITH_STROKE),
    "nleq" => Token::Operator(ops::NEITHER_LESS_THAN_NOR_EQUAL_TO),
    "nless" => Token::Operator(ops::NOT_LESS_THAN),
    "nlessgt" => Token::Operator(ops::NEITHER_LESS_THAN_NOR_GREATER_THAN),
    "nlesssim" => Token::Operator(ops::NEITHER_LESS_THAN_NOR_EQUIVALENT_TO),
    "nmid" => Token::Operator(ops::DOES_NOT_DIVIDE),
    "not" => Token::Not,
    "notin" => Token::Operator(ops::NOT_AN_ELEMENT_OF),
    "nparallel" => Token::Operator(ops::NOT_PARALLEL_TO),
    "nprec" => Token::Operator(ops::DOES_NOT_PRECEDE),
    "npreceq" => Token::Operator(ops::DOES_NOT_PRECEDE_OR_EQUAL),
    "nrightarrow" => Token::Operator(ops::RIGHTWARDS_ARROW_WITH_STROKE),
    "nsim" => Token::Operator(ops::NOT_TILDE),
    "nsubset" => Token::Operator(ops::NOT_A_SUBSET_OF),
    "nsubseteq" => Token::Operator(ops::NEITHER_A_SUBSET_OF_NOR_EQUAL_TO),
    "nsucc" => Token::Operator(ops::DOES_NOT_SUCCEED),
    "nsucceq" => Token::Operator(ops::DOES_NOT_SUCCEED_OR_EQUAL),
    "nsupset" => Token::Operator(ops::NOT_A_SUPERSET_OF),
    "nsupseteq" => Token::Operator(ops::NEITHER_A_SUPERSET_OF_NOR_EQUAL_TO),
    "nu" => Token::Letter('ν'),
    "nwarrow" => Token::Operator(ops::NORTH_WEST_ARROW),
    "o" => Token::Letter('ø'),
    "odot" => Token::Operator(ops::CIRCLED_DOT_OPERATOR),
    "oe" => Token::Letter('œ'),
    "oiiint" => Token::Integral(ops::VOLUME_INTEGRAL),
    "oiint" => Token::Integral(ops::SURFACE_INTEGRAL),
    "oint" => Token::Integral(ops::CONTOUR_INTEGRAL),
    "ointctrclockwise" => Token::Integral(ops::ANTICLOCKWISE_CONTOUR_INTEGRAL),
    "omega" => Token::Letter('ω'),
    "omicron" => Token::Letter('ο'),
    "ominus" => Token::Operator(ops::CIRCLED_MINUS),
    "operatorname" => Token::OperatorName,
    "oplus" => Token::Operator(ops::CIRCLED_PLUS),
    "oslash" => Token::Operator(ops::CIRCLED_DIVISION_SLASH),
    "otimes" => Token::Operator(ops::CIRCLED_TIMES),
    "overbrace" => Token::OverUnderBrace(ops::TOP_CURLY_BRACKET, true),
    "overbracket" => Token::OverUnderBrace(ops::TOP_SQUARE_BRACKET, true),
    "overleftarrow" => Token::OverUnder(ops::LEFTWARDS_ARROW, true, None),
    "overline" => Token::OverUnder(ops::OVERLINE, true, None),
    "overparen" => Token::OverUnderBrace(ops::TOP_PARENTHESIS, true),
    "overrightarrow" => Token::OverUnder(ops::RIGHTWARDS_ARROW, true, None),
    "overset" => Token::Overset,
    "parallel" => Token::Operator(ops::PARALLEL_TO),
    "partial" => Token::Letter(ops::PARTIAL_DIFFERENTIAL),
    "perp" => Token::Operator(ops::UP_TACK),
    "phi" => Token::Letter('ϕ'),
    "pi" => Token::Letter('π'),
    "pm" => Token::Operator(ops::PLUS_MINUS_SIGN),
    "pounds" => Token::Letter('£'),
    "prec" => Token::Operator(ops::PRECEDES),
    "precapprox" => Token::Operator(ops::PRECEDES_ABOVE_ALMOST_EQUAL_TO),
    "preccurlyeq" => Token::Operator(ops::PRECEDES_OR_EQUAL_TO),
    "preceq" => Token::Operator(ops::PRECEDES_ABOVE_SINGLE_LINE_EQUALS_SIGN),
    "precnapprox" => Token::Operator(ops::PRECEDES_ABOVE_NOT_ALMOST_EQUAL_TO),
    "precneqq" => Token::Operator(ops::PRECEDES_ABOVE_NOT_EQUAL_TO),
    "precnsim" => Token::Operator(ops::PRECEDES_BUT_NOT_EQUIVALENT_TO),
    "precsim" => Token::Operator(ops::PRECEDES_OR_EQUIVALENT_TO),
    "prime" => Token::Operator(ops::PRIME),
    "prod" => Token::BigOp(ops::N_ARY_PRODUCT),
    "propto" => Token::Operator(ops::PROPORTIONAL_TO),
    "psi" => Token::Letter('ψ'),
    "qprime" => Token::Operator(ops::QUADRUPLE_PRIME),
    "qquad" => Token::Space("2"),
    "quad" => Token::Space("1"),
    "questeq" => Token::Operator(ops::QUESTIONED_EQUAL_TO), // from "stix"
    "rVert" => Token::Paren(ops::DOUBLE_VERTICAL_LINE, None, Stretchy::PrePostfix),
    "rangle" => Token::Paren(ops::MATHEMATICAL_RIGHT_ANGLE_BRACKET, None, Stretchy::Always),
    "rbrace" => Token::Paren(ops::RIGHT_CURLY_BRACKET, None, Stretchy::Always),
    "rbrack" => Token::Paren(ops::RIGHT_SQUARE_BRACKET, None, Stretchy::Always),
    "rceil" => Token::Paren(ops::RIGHT_CEILING, None, Stretchy::Always),
    "rfloor" => Token::Paren(ops::RIGHT_FLOOR, None, Stretchy::Always),
    "rgroup" => Token::Paren(ops::MATHEMATICAL_RIGHT_FLATTENED_PARENTHESIS, None, Stretchy::Always),
    "rhd" => Token::Operator(ops::CONTAINS_AS_NORMAL_SUBGROUP),
    "rho" => Token::Letter('ρ'),
    "right" => Token::Right,
    "rightarrow" => Token::Operator(ops::RIGHTWARDS_ARROW),
    "rightarrowtail" => Token::Operator(ops::RIGHTWARDS_ARROW_WITH_TAIL),
    "rightharpoondown" => Token::Operator(ops::RIGHTWARDS_HARPOON_WITH_BARB_DOWNWARDS),
    "rightharpoonup" => Token::Operator(ops::RIGHTWARDS_HARPOON_WITH_BARB_UPWARDS),
    "rightleftarrows" => Token::Operator(ops::RIGHTWARDS_ARROW_OVER_LEFTWARDS_ARROW),
    "rightleftharpoons" => Token::Operator(ops::RIGHTWARDS_HARPOON_OVER_LEFTWARDS_HARPOON),
    "rightrightarrows" => Token::Operator(ops::RIGHTWARDS_PAIRED_ARROWS),
    "rightsquigarrow" => Token::Operator(ops::RIGHTWARDS_SQUIGGLE_ARROW),
    "rightthreetimes" => Token::Operator(ops::RIGHT_SEMIDIRECT_PRODUCT),
    "risingdotseq" => Token::Operator(ops::IMAGE_OF_OR_APPROXIMATELY_EQUAL_TO),
    "rq" => Token::Letter('’'),
    "rrbracket" => Token::Paren(ops::MATHEMATICAL_RIGHT_WHITE_SQUARE_BRACKET, None, Stretchy::Always),
    "rtimes" => Token::Operator(ops::RIGHT_NORMAL_FACTOR_SEMIDIRECT_PRODUCT),
    "rupee" => Token::Letter('₹'),
    "rvert" => Token::Paren(ops::VERTICAL_LINE, None, Stretchy::PrePostfix),
    "saturn" => Token::Letter('♄'),
    "scriptscriptstyle" => Token::Style(Style::ScriptScriptStyle),
    "scriptstyle" => Token::Style(Style::ScriptStyle),
    "searrow" => Token::Operator(ops::SOUTH_EAST_ARROW),
    "sec" => Token::Function("sec"),
    "setminus" => Token::Operator(ops::SET_MINUS),
    "sgn" => Token::Function("sgn"),
    "sharp" => Token::Letter('♯'),
    "sigma" => Token::Letter('σ'),
    "sim" => Token::Operator(ops::TILDE_OPERATOR),
    "simeq" => Token::Operator(ops::ASYMPTOTICALLY_EQUAL_TO),
    "sin" => Token::Function("sin"),
    "sinh" => Token::Function("sinh"),
    "slashed" => Token::Slashed,
    "smallsetminus" => Token::Operator(ops::SMALL_REVERSE_SOLIDUS),
    "smile" => Token::Operator(ops::SMILE),
    "spadesuit" => Token::Letter('♠'),
    "sphericalangle" => Token::Letter(ops::SPHERICAL_ANGLE),
    "sqcap" => Token::Operator(ops::SQUARE_CAP),
    "sqcup" => Token::Operator(ops::SQUARE_CUP),
    "sqrt" => Token::Sqrt,
    "sqsubset" => Token::Operator(ops::SQUARE_IMAGE_OF),
    "sqsubseteq" => Token::Operator(ops::SQUARE_IMAGE_OF_OR_EQUAL_TO),
    "sqsupset" => Token::Operator(ops::SQUARE_ORIGINAL_OF),
    "sqsupseteq" => Token::Operator(ops::SQUARE_ORIGINAL_OF_OR_EQUAL_TO),
    "square" => Token::Letter('□'),
    "ss" => Token::Letter('ß'),
    "star" => Token::Operator(ops::STAR_OPERATOR),
    "stareq" => Token::Operator(ops::STAR_EQUALS), // from "stix"
    "subset" => Token::Operator(ops::SUBSET_OF),
    "subseteq" => Token::Operator(ops::SUBSET_OF_OR_EQUAL_TO),
    "subsetneq" => Token::Operator(ops::SUBSET_OF_WITH_NOT_EQUAL_TO),
    "subsetneqq" => Token::Operator(ops::SUBSET_OF_ABOVE_NOT_EQUAL_TO),
    "succ" => Token::Operator(ops::SUCCEEDS),
    "succapprox" => Token::Operator(ops::SUCCEEDS_ABOVE_ALMOST_EQUAL_TO),
    "succcurlyeq" => Token::Operator(ops::SUCCEEDS_OR_EQUAL_TO),
    "succeq" => Token::Operator(ops::SUCCEEDS_ABOVE_SINGLE_LINE_EQUALS_SIGN),
    "succnapprox" => Token::Operator(ops::SUCCEEDS_ABOVE_NOT_ALMOST_EQUAL_TO),
    "succneqq" => Token::Operator(ops::SUCCEEDS_ABOVE_NOT_EQUAL_TO),
    "succnsim" => Token::Operator(ops::SUCCEEDS_BUT_NOT_EQUIVALENT_TO),
    "succsim" => Token::Operator(ops::SUCCEEDS_OR_EQUIVALENT_TO),
    "sum" => Token::BigOp(ops::N_ARY_SUMMATION),
    "sumint" => Token::BigOp(ops::SUMMATION_WITH_INTEGRAL),
    "sun" => Token::Letter('☼'),
    "sup" => Token::Lim("sup"),
    "supset" => Token::Operator(ops::SUPERSET_OF),
    "supseteq" => Token::Operator(ops::SUPERSET_OF_OR_EQUAL_TO),
    "supsetneq" => Token::Operator(ops::SUPERSET_OF_WITH_NOT_EQUAL_TO),
    "supsetneqq" => Token::Operator(ops::SUPERSET_OF_ABOVE_NOT_EQUAL_TO),
    "swarrow" => Token::Operator(ops::SOUTH_WEST_ARROW),
    "symbf" => Token::Transform(Some(TextTransform::BoldItalic), None),
    "tan" => Token::Function("tan"),
    "tanh" => Token::Function("tanh"),
    "tau" => Token::Letter('τ'),
    "tbinom" => Token::Binom(Some(FracAttr::DisplayStyleFalse)),
    "text" => Token::Text(None),
    "textbf" => Token::Text(Some(TextTransform::Bold)),
    "textit" => Token::Text(Some(TextTransform::Italic)),
    "textstyle" => Token::Style(Style::TextStyle),
    "texttt" => Token::Text(Some(TextTransform::Monospace)),
    "textyen" => Token::Letter('¥'),
    "tfrac" => Token::Frac(Some(FracAttr::DisplayStyleFalse)),
    "th" => Token::Letter('þ'),
    "therefore" => Token::Operator(ops::THEREFORE),
    "theta" => Token::Letter('θ'),
    "tilde" => Token::OverUnder(ops::TILDE, true, Some(OpAttr::StretchyFalse)),
    "times" => Token::Operator(ops::MULTIPLICATION_SIGN),
    "to" => Token::Operator(ops::RIGHTWARDS_ARROW),
    "top" => Token::Operator(ops::DOWN_TACK),
    "triangle" => Token::Letter('△'),
    "triangledown" => Token::Operator(ops::WHITE_DOWN_POINTING_TRIANGLE),
    "triangleleft" => Token::Operator(ops::WHITE_LEFT_POINTING_TRIANGLE),
    "triangleq" => Token::Operator(ops::DELTA_EQUAL_TO),
    "triangleright" => Token::Operator(ops::WHITE_RIGHT_POINTING_TRIANGLE),
    "trprime" => Token::Operator(ops::TRIPLE_PRIME),
    "ulcorner" => Token::Letter(ops::TOP_LEFT_CORNER),
    "underbrace" => Token::OverUnderBrace(ops::BOTTOM_CURLY_BRACKET, false),
    "underbracket" => Token::OverUnderBrace(ops::BOTTOM_SQUARE_BRACKET, false),
    "underline" => Token::OverUnder(ops::LOW_LINE, false, None),
    "underparen" => Token::OverUnderBrace(ops::BOTTOM_PARENTHESIS, false),
    "underset" => Token::Underset,
    "unlhd" => Token::Operator(ops::NORMAL_SUBGROUP_OF_OR_EQUAL_TO),
    "unrhd" => Token::Operator(ops::CONTAINS_AS_NORMAL_SUBGROUP_OR_EQUAL_TO),
    "uparrow" => Token::Paren(ops::UPWARDS_ARROW, None, Stretchy::Inconsistent),
    "updownarrow" => Token::Paren(ops::UP_DOWN_ARROW, None, Stretchy::Inconsistent),
    "upharpoonleft" => Token::Operator(ops::UPWARDS_HARPOON_WITH_BARB_LEFTWARDS),
    "upharpoonright" => Token::Operator(ops::UPWARDS_HARPOON_WITH_BARB_RIGHTWARDS),
    "uplus" => Token::Operator(ops::MULTISET_UNION),
    "upsilon" => Token::Letter('υ'),
    "upuparrows" => Token::Operator(ops::UPWARDS_PAIRED_ARROWS),
    "uranus" => Token::Letter('♅'),
    "urcorner" => Token::Letter(ops::TOP_RIGHT_CORNER),
    "vDash" => Token::Operator(ops::TRUE),
    "varDelta" => Token::Letter('Δ'), // not italicized
    "varGamma" => Token::Letter('Γ'), // not italicized
    "varLambda" => Token::Letter('Λ'), // not italicized
    "varOmega" => Token::Letter('Ω'), // not italicized
    "varPhi" => Token::Letter('Φ'), // not italicized
    "varPi" => Token::Letter('Π'), // not italicized
    "varSigma" => Token::Letter('Σ'), // not italicized
    "varTheta" => Token::Letter('Θ'), // not italicized
    "varUpsilon" => Token::Letter('ϒ'), // not italicized
    "varXi" => Token::Letter('Ξ'), // not italicized
    "varepsilon" => Token::Letter('ε'),
    "varkappa" => Token::Letter('ϰ'),
    "varnothing" => Token::Letter('⌀'),
    "varointclockwise" => Token::Operator(ops::CLOCKWISE_CONTOUR_INTEGRAL),
    "varphi" => Token::Letter('φ'),
    "varpi" => Token::Letter('ϖ'),
    "varrho" => Token::Letter('ϱ'),
    "varsigma" => Token::Letter('ς'),
    "vartheta" => Token::Letter('ϑ'),
    "vartriangle" => Token::Operator(ops::WHITE_UP_POINTING_TRIANGLE),
    "vdash" => Token::Operator(ops::RIGHT_TACK),
    "vdots" => Token::Operator(ops::VERTICAL_ELLIPSIS),
    "vec" => Token::OverUnder(ops::RIGHTWARDS_ARROW, true, Some(OpAttr::StretchyFalse)),
    "vee" => Token::Operator(ops::LOGICAL_OR),
    "veebar" => Token::Operator(ops::XOR),
    "veeeq" => Token::Operator(ops::EQUIANGULAR_TO), // from "stix"
    "venus" => Token::Letter('♀'),
    "vert" => Token::Paren(ops::VERTICAL_LINE, None, Stretchy::PrePostfix),
    "wedge" => Token::Operator(ops::LOGICAL_AND),
    "wedgeq" => Token::Operator(ops::ESTIMATES), // from "stix"
    "widehat" => Token::OverUnder(ops::CIRCUMFLEX_ACCENT, true, None),
    "widetilde" => Token::OverUnder(ops::TILDE, true, None),
    "wp" => Token::Function("℘"),
    "wr" => Token::Operator(ops::WREATH_PRODUCT),
    "xi" => Token::Letter('ξ'),
    "zeta" => Token::Letter('ζ'),
    "{" => Token::Paren(ops::LEFT_CURLY_BRACKET, None, Stretchy::Always),
    "|" => Token::Paren(ops::DOUBLE_VERTICAL_LINE, None, Stretchy::PrePostfix),
    "}" => Token::Paren(ops::RIGHT_CURLY_BRACKET, None, Stretchy::Always),
};

pub fn get_command(command: &str) -> Token<'_> {
    match COMMANDS.get(command) {
        Some(token) => *token,
        None => Token::UnknownCommand(command),
    }
}

pub fn get_negated_op(op: Op) -> Option<Op> {
    match op {
        ops::ALMOST_EQUAL_TO => Some(ops::NOT_ALMOST_EQUAL_TO),
        ops::APPROXIMATELY_EQUAL_TO => Some(ops::NOT_ASYMPTOTICALLY_EQUAL_TO),
        ops::ELEMENT_OF => Some(ops::NOT_AN_ELEMENT_OF),
        ops::GREATER_THAN_OVER_EQUAL_TO => Some(ops::NEITHER_GREATER_THAN_NOR_EQUAL_TO),
        ops::LESS_THAN_OR_EQUAL_TO => Some(ops::NEITHER_LESS_THAN_NOR_EQUAL_TO),
        ops::PRECEDES => Some(ops::DOES_NOT_PRECEDE),
        ops::SUBSET_OF => Some(ops::NOT_A_SUBSET_OF),
        ops::SUBSET_OF_OR_EQUAL_TO => Some(ops::NEITHER_A_SUBSET_OF_NOR_EQUAL_TO),
        ops::SUCCEEDS => Some(ops::DOES_NOT_SUCCEED),
        ops::SUPERSET_OF => Some(ops::NOT_A_SUPERSET_OF),
        ops::SUPERSET_OF_OR_EQUAL_TO => Some(ops::NEITHER_A_SUPERSET_OF_NOR_EQUAL_TO),
        _ => None,
    }
}
