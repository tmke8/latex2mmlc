use crate::attribute::{FracAttr, Style, TextTransform};
use crate::ops::{self, Op};
use crate::token::Token;

static COMMANDS: phf::Map<&'static str, Token> = phf::phf_map! {
    " " => Token::Space("1"),
    "!" => Token::Space("-0.1667"),
    "#" => Token::NormalLetter('#'),
    "$" => Token::NormalLetter('$'),
    "%" => Token::NormalLetter('%'),
    "&" => Token::OpAmpersand,
    "," => Token::Space("0.1667"),
    ":" => Token::Space("0.2222"),
    ";" => Token::Space("0.2778"),
    "A" => Token::NormalLetter('Å'),
    "AE" => Token::NormalLetter('Æ'),
    "Alpha" => Token::NormalLetter('Α'),
    "And" => Token::OpAmpersand,
    "Beta" => Token::NormalLetter('Β'),
    "Big" => Token::Big("1.623em"),
    "Bigg" => Token::Big("2.470em"),
    "Biggl" => Token::Big("2.470em"),
    "Biggr" => Token::Big("2.470em"),
    "Bigl" => Token::Big("1.623em"),
    "Bigr" => Token::Big("1.623em"),
    "Box" => Token::NormalLetter('◻'),
    "Cap" => Token::Operator(ops::DOUBLE_INTERSECTION),
    "Chi" => Token::NormalLetter('Χ'),
    "Cup" => Token::Operator(ops::DOUBLE_UNION),
    "DH" => Token::NormalLetter('Ð'),
    "Dagger" => Token::NormalLetter('‡'),
    "Delta" => Token::NormalLetter('Δ'),
    "Diamond" => Token::NormalLetter('◊'),
    "Doteq" => Token::Operator(ops::GEOMETRICALLY_EQUAL_TO),
    "Downarrow" => Token::Operator(ops::DOWNWARDS_DOUBLE_ARROW),
    "Epsilon" => Token::NormalLetter('Ε'),
    "Eta" => Token::NormalLetter('Η'),
    "Finv" => Token::NormalLetter('Ⅎ'),
    "Game" => Token::NormalLetter('⅁'),
    "Gamma" => Token::NormalLetter('Γ'),
    "Im" => Token::NormalLetter('ℑ'),
    "Iota" => Token::NormalLetter('Ι'),
    "Join" => Token::Operator(ops::BOWTIE),
    "Kappa" => Token::NormalLetter('Κ'),
    "L" => Token::NormalLetter('Ł'),
    "Lambda" => Token::NormalLetter('Λ'),
    "Leftarrow" => Token::Operator(ops::LEFTWARDS_DOUBLE_ARROW),
    "Leftrightarrow" => Token::Operator(ops::LEFT_RIGHT_DOUBLE_ARROW),
    "Lleftarrow" => Token::Operator(ops::LEFTWARDS_TRIPLE_ARROW),
    "Longleftarrow" => Token::Operator(ops::LONG_LEFTWARDS_DOUBLE_ARROW),
    "Longleftrightarrow" => Token::Operator(ops::LONG_LEFT_RIGHT_DOUBLE_ARROW),
    "Longrightarrow" => Token::Operator(ops::LONG_RIGHTWARDS_DOUBLE_ARROW),
    "Lsh" => Token::Operator(ops::UPWARDS_ARROW_WITH_TIP_LEFTWARDS),
    "Mu" => Token::NormalLetter('Μ'),
    "NG" => Token::NormalLetter('Ŋ'),
    "Nu" => Token::NormalLetter('Ν'),
    "O" => Token::NormalLetter('Ø'),
    "OE" => Token::NormalLetter('Œ'),
    "Omega" => Token::NormalLetter('Ω'),
    "Omicron" => Token::NormalLetter('Ο'),
    "P" => Token::NormalLetter('¶'),
    "Phi" => Token::NormalLetter('Φ'),
    "Pi" => Token::NormalLetter('Π'),
    "Psi" => Token::NormalLetter('Ψ'),
    "Re" => Token::NormalLetter('ℜ'),
    "Rho" => Token::NormalLetter('Ρ'),
    "Rightarrow" => Token::Operator(ops::RIGHTWARDS_DOUBLE_ARROW),
    "Rrightarrow" => Token::Operator(ops::RIGHTWARDS_TRIPLE_ARROW),
    "Rsh" => Token::Operator(ops::UPWARDS_ARROW_WITH_TIP_RIGHTWARDS),
    "S" => Token::NormalLetter('§'),
    "Sigma" => Token::NormalLetter('Σ'),
    "TH" => Token::NormalLetter('Þ'),
    "Tau" => Token::NormalLetter('Τ'),
    "Theta" => Token::NormalLetter('Θ'),
    "Uparrow" => Token::Operator(ops::UPWARDS_DOUBLE_ARROW),
    "Updownarrow" => Token::Operator(ops::UP_DOWN_DOUBLE_ARROW),
    "Upsilon" => Token::NormalLetter('Υ'),
    "Vdash" => Token::Operator(ops::FORCES),
    "Xi" => Token::NormalLetter('Ξ'),
    "Yleft" => Token::Operator(ops::LEFTWARDS_ARROW_TAIL),
    "Yright" => Token::Operator(ops::RIGHTWARDS_ARROW_TAIL),
    "Zeta" => Token::NormalLetter('Ζ'),
    "\\" => Token::NewLine,
    "_" => Token::NormalLetter('_'),
    "a" => Token::NormalLetter('å'),
    "acute" => Token::Over(ops::ACUTE_ACCENT),
    "ae" => Token::NormalLetter('æ'),
    "aleph" => Token::NormalLetter('ℵ'),
    "alpha" => Token::Letter('α'),
    "amalg" => Token::Operator(ops::AMALGAMATION_OR_COPRODUCT),
    "angle" => Token::NormalLetter('∠'),
    "approx" => Token::Operator(ops::ALMOST_EQUAL_TO),
    "approxeq" => Token::Operator(ops::ALMOST_EQUAL_OR_EQUAL_TO),
    "arccos" => Token::Function("arccos"),
    "arcsin" => Token::Function("arcsin"),
    "arctan" => Token::Function("arctan"),
    "arg" => Token::Function("arg"),
    "ascnode" => Token::NormalLetter('☊'),
    "ast" => Token::Operator(ops::ASTERISK_OPERATOR),
    "astrosun" => Token::NormalLetter('☉'),
    "asymp" => Token::Operator(ops::EQUIVALENT_TO),
    "backslash" => Token::Operator(ops::REVERSE_SOLIDUS),
    "bar" => Token::Over(ops::MACRON),
    "barwedge" => Token::Operator(ops::NAND),
    "because" => Token::NormalLetter('∵'),
    "begin" => Token::Begin,
    "beta" => Token::Letter('β'),
    "beth" => Token::NormalLetter('ℶ'),
    "big" => Token::Big("1.2em"),
    "bigcap" => Token::BigOp(ops::N_ARY_INTERSECTION),
    "bigcirc" => Token::Operator(ops::LARGE_CIRCLE),
    "bigcup" => Token::BigOp(ops::N_ARY_UNION),
    "bigg" => Token::Big("2.047em"),
    "biggl" => Token::Big("2.047em"),
    "biggr" => Token::Big("2.047em"),
    "bigl" => Token::Big("1.2em"),
    "bigodot" => Token::BigOp(ops::N_ARY_CIRCLED_DOT_OPERATOR),
    "bigoplus" => Token::BigOp(ops::N_ARY_CIRCLED_PLUS_OPERATOR),
    "bigr" => Token::Big("1.2em"),
    "bigsqcup" => Token::BigOp(ops::N_ARY_SQUARE_UNION_OPERATOR),
    "bigtimes" => Token::BigOp(ops::N_ARY_TIMES_OPERATOR),
    "bigtriangleup" => Token::NormalLetter('△'),
    "biguplus" => Token::BigOp(ops::N_ARY_UNION_OPERATOR_WITH_PLUS),
    "bigvee" => Token::BigOp(ops::N_ARY_LOGICAL_OR),
    "bigwedge" => Token::BigOp(ops::N_ARY_LOGICAL_AND),
    "binom" => Token::Binom(None),
    "bitotimes" => Token::BigOp(ops::N_ARY_CIRCLED_TIMES_OPERATOR),
    "bm" => Token::Transform(TextTransform::BoldItalic),
    "boldsymbol" => Token::Transform(TextTransform::BoldItalic),
    "bot" => Token::Operator(ops::UP_TACK),
    "botdoteq" => Token::Operator(ops::EQUALS_SIGN_WITH_DOT_BELOW),
    "boxbox" => Token::Operator(ops::SQUARED_SQUARE),
    "boxbslash" => Token::Operator(ops::SQUARED_FALLING_DIAGONAL_SLASH),
    "boxdot" => Token::Operator(ops::SQUARED_DOT_OPERATOR),
    "boxminus" => Token::Operator(ops::SQUARED_MINUS),
    "boxplus" => Token::Operator(ops::SQUARED_PLUS),
    "boxslash" => Token::Operator(ops::SQUARED_RISING_DIAGONAL_SLASH),
    "boxtimes" => Token::Operator(ops::SQUARED_TIMES),
    "breve" => Token::Over(ops::BREVE),
    "bullet" => Token::Operator(ops::BULLET_OPERATOR),
    "cap" => Token::Operator(ops::INTERSECTION),
    "cdot" => Token::Operator(ops::MIDDLE_DOT),
    "cdots" => Token::Operator(ops::MIDLINE_HORIZONTAL_ELLIPSIS),
    "centerdot" => Token::Operator(ops::BULLET_OPERATOR),
    "cfrac" => Token::Frac(Some(FracAttr::CFracStyle)),
    "check" => Token::Over(ops::CARON),
    "checkmark" => Token::NormalLetter('✓'),
    "chi" => Token::Letter('χ'),
    "circ" => Token::Operator(ops::RING_OPERATOR),
    "circeq" => Token::Operator(ops::RING_EQUAL_TO),
    "circlearrowleft" => Token::Operator(ops::ANTICLOCKWISE_OPEN_CIRCLE_ARROW),
    "circlearrowright" => Token::Operator(ops::CLOCKWISE_OPEN_CIRCLE_ARROW),
    "circledR" => Token::NormalLetter('Ⓡ'),
    "circledast" => Token::Operator(ops::CIRCLED_ASTERISK_OPERATOR),
    "circledcirc" => Token::Operator(ops::CIRCLED_RING_OPERATOR),
    "circleddash" => Token::Operator(ops::CIRCLED_DASH),
    "clubsuit" => Token::NormalLetter('♣'),
    "colon" => Token::NormalLetter(':'),
    "coloneq" => Token::Operator(ops::COLON_EQUALS),
    "complement" => Token::NormalLetter('∁'),
    "cong" => Token::Operator(ops::APPROXIMATELY_EQUAL_TO),
    "coprod" => Token::BigOp(ops::N_ARY_COPRODUCT),
    "copyright" => Token::NormalLetter('©'),
    "cos" => Token::Function("cos"),
    "cosh" => Token::Function("cosh"),
    "cot" => Token::Function("cot"),
    "coth" => Token::Function("coth"),
    "csc" => Token::Function("csc"),
    "cup" => Token::Operator(ops::UNION),
    "curlyvee" => Token::Operator(ops::CURLY_LOGICAL_OR),
    "curlywedge" => Token::Operator(ops::CURLY_LOGICAL_AND),
    "curvearrowleft" => Token::Operator(ops::ANTICLOCKWISE_TOP_SEMICIRCLE_ARROW),
    "curvearrowright" => Token::Operator(ops::CLOCKWISE_TOP_SEMICIRCLE_ARROW),
    "dag" => Token::NormalLetter('†'),
    "dagger" => Token::NormalLetter('†'),
    "daleth" => Token::NormalLetter('ℸ'),
    "dashv" => Token::Operator(ops::LEFT_TACK),
    "dbinom" => Token::Binom(Some(FracAttr::DisplayStyleTrue)),
    "ddag" => Token::NormalLetter('‡'),
    "ddot" => Token::Over(ops::DIAERESIS),
    "ddots" => Token::Operator(ops::DOWN_RIGHT_DIAGONAL_ELLIPSIS),
    "delta" => Token::Letter('δ'),
    "det" => Token::Function("det"),
    "dfrac" => Token::Frac(Some(FracAttr::DisplayStyleTrue)),
    "dh" => Token::NormalLetter('ð'),
    "diamondsuit" => Token::NormalLetter('♢'),
    "digamma" => Token::Letter('ϝ'),
    "dim" => Token::Function("dim"),
    "displaystyle" => Token::Style(Style::DisplayStyle),
    "div" => Token::Operator(ops::DIVISION_SIGN),
    "divideontimes" => Token::Operator(ops::DIVISION_TIMES),
    "dj" => Token::NormalLetter('đ'),
    "dot" => Token::Over(ops::DOT_ABOVE),
    "doteq" => Token::Operator(ops::APPROACHES_THE_LIMIT),
    "doteqdot" => Token::Operator(ops::GEOMETRICALLY_EQUAL_TO),
    "dotplus" => Token::Operator(ops::DOT_PLUS),
    "dots" => Token::Operator(ops::MIDLINE_HORIZONTAL_ELLIPSIS),
    "downarrow" => Token::Paren(ops::DOWNWARDS_ARROW),
    "downdownarrows" => Token::Operator(ops::DOWNWARDS_PAIRED_ARROWS),
    "downharpoonleft" => Token::Operator(ops::DOWNWARDS_HARPOON_WITH_BARB_LEFTWARDS),
    "downharpoonright" => Token::Operator(ops::DOWNWARDS_HARPOON_WITH_BARB_RIGHTWARDS),
    "earth" => Token::NormalLetter('♁'),
    "ell" => Token::Letter('ℓ'),
    "emptyset" => Token::NormalLetter('∅'),
    "end" => Token::End,
    "epsilon" => Token::Letter('ϵ'),
    "eqcirc" => Token::Operator(ops::RING_IN_EQUAL_TO),
    "eqcolon" => Token::Operator(ops::EQUALS_COLON),
    "eqslantgtr" => Token::Operator(ops::SLANTED_EQUAL_TO_OR_GREATER_THAN),
    "eqslantless" => Token::Operator(ops::SLANTED_EQUAL_TO_OR_LESS_THAN),
    "equiv" => Token::Operator(ops::IDENTICAL_TO),
    "erf" => Token::Function("erf"),
    "erfc" => Token::Function("erfc"),
    "eta" => Token::Letter('η'),
    "eth" => Token::NormalLetter('ð'),
    "euro" => Token::NormalLetter('€'),
    "exists" => Token::Operator(ops::THERE_EXISTS),
    "exp" => Token::Function("exp"),
    "fallingdotseq" => Token::Operator(ops::APPROXIMATELY_EQUAL_TO_OR_THE_IMAGE_OF),
    "flat" => Token::NormalLetter('♭'),
    "forall" => Token::Operator(ops::FOR_ALL),
    "frac" => Token::Frac(None),
    "frown" => Token::Operator(ops::FROWN),
    "gamma" => Token::Letter('γ'),
    "ge" => Token::Operator(ops::GREATER_THAN_OR_EQUAL_TO),
    "genfrac" => Token::Genfrac,
    "geq" => Token::Operator(ops::GREATER_THAN_OR_EQUAL_TO),
    "geqq" => Token::Operator(ops::GREATER_THAN_OVER_EQUAL_TO),
    "geqslant" => Token::Operator(ops::GREATER_THAN_OR_SLANTED_EQUAL_TO),
    "gets" => Token::Operator(ops::LEFTWARDS_ARROW),
    "gg" => Token::Operator(ops::MUCH_GREATER_THAN),
    "gimel" => Token::NormalLetter('ℷ'),
    "grave" => Token::Over(ops::GRAVE_ACCENT),
    "gt" => Token::OpGreaterThan,
    "gtrapprox" => Token::Operator(ops::GREATER_THAN_OR_APPROXIMATE),
    "gtrsim" => Token::Operator(ops::GREATER_THAN_OR_EQUIVALENT_TO),
    "hat" => Token::Over(ops::CIRCUMFLEX_ACCENT),
    "hbar" => Token::Letter('ℏ'),
    "heartsuit" => Token::NormalLetter('♡'),
    "hookleftarrow" => Token::Operator(ops::LEFTWARDS_ARROW_WITH_HOOK),
    "hookrightarrow" => Token::Operator(ops::RIGHTWARDS_ARROW_WITH_HOOK),
    "hslash" => Token::Letter('ℏ'),
    "iff" => Token::Operator(ops::LONG_LEFT_RIGHT_DOUBLE_ARROW),
    "iiint" => Token::Integral(ops::TRIPLE_INTEGRAL),
    "iint" => Token::Integral(ops::DOUBLE_INTEGRAL),
    "imath" => Token::Letter('ı'),
    "impliedby" => Token::Operator(ops::LONG_LEFTWARDS_DOUBLE_ARROW),
    "implies" => Token::Operator(ops::LONG_RIGHTWARDS_DOUBLE_ARROW),
    "in" => Token::Operator(ops::ELEMENT_OF),
    "inf" => Token::Lim("inf"),
    "infty" => Token::Letter('∞'),
    "int" => Token::Integral(ops::INTEGRAL),
    "intercal" => Token::Operator(ops::INTERCALATE),
    "iota" => Token::Letter('ι'),
    "jmath" => Token::Letter('ȷ'),
    "jupiter" => Token::NormalLetter('♃'),
    "kappa" => Token::Letter('κ'),
    "ker" => Token::Function("ker"),
    "l" => Token::NormalLetter('ł'),
    "lambda" => Token::Letter('λ'),
    "land" => Token::Operator(ops::LOGICAL_AND),
    "langle" => Token::Paren(ops::MATHEMATICAL_LEFT_ANGLE_BRACKET),
    "lceil" => Token::Paren(ops::LEFT_CEILING),
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
    "lfloor" => Token::Paren(ops::LEFT_FLOOR),
    "lgroup" => Token::Paren(ops::MATHEMATICAL_LEFT_FLATTENED_PARENTHESIS),
    "lhd" => Token::Operator(ops::NORMAL_SUBGROUP_OF),
    "lightning" => Token::Operator(ops::DOWNWARDS_ZIGZAG_ARROW),
    "lim" => Token::Lim("lim"),
    "liminf" => Token::Lim("lim inf"),
    "limits" => Token::Limits,
    "limsup" => Token::Lim("lim sup"),
    "ll" => Token::Operator(ops::MUCH_LESS_THAN),
    "llbracket" => Token::Paren(ops::MATHEMATICAL_LEFT_WHITE_SQUARE_BRACKET),
    "lll" => Token::Operator(ops::VERY_MUCH_LESS_THAN),
    "ln" => Token::Function("ln"),
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
    "lt" => Token::OpLessThan,
    "ltimes" => Token::Operator(ops::LEFT_NORMAL_FACTOR_SEMIDIRECT_PRODUCT),
    "lvert" => Token::Paren(ops::VERTICAL_LINE),
    "maltese" => Token::Letter('✠'),
    "mapsto" => Token::Operator(ops::RIGHTWARDS_ARROW_FROM_BAR),
    "mars" => Token::Letter('♂'),
    "mathbb" => Token::Transform(TextTransform::DoubleStruck),
    "mathbf" => Token::Transform(TextTransform::Bold),
    "mathcal" => Token::Transform(TextTransform::Script),
    "mathfrak" => Token::Transform(TextTransform::Fraktur),
    "mathit" => Token::Transform(TextTransform::Italic),
    "mathrm" => Token::NormalVariant,
    "mathscr" => Token::Transform(TextTransform::Script),
    "mathsf" => Token::Transform(TextTransform::SansSerif),
    "mathstrut" => Token::Mathstrut,
    "max" => Token::Lim("max"),
    "mercury" => Token::Letter('☿'),
    "mho" => Token::NormalLetter('℧'),
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
    "nabla" => Token::Operator(ops::NABLA),
    "natural" => Token::NormalLetter('♮'),
    "ne" => Token::Operator(ops::NOT_EQUAL_TO),
    "nearrow" => Token::Operator(ops::NORTH_EAST_ARROW),
    "neg" => Token::Operator(ops::NOT_SIGN),
    "neptune" => Token::NormalLetter('♆'),
    "neq" => Token::Operator(ops::NOT_EQUAL_TO),
    "nequiv" => Token::Operator(ops::NOT_IDENTICAL_TO),
    "nexists" => Token::Operator(ops::THERE_DOES_NOT_EXIST),
    "ng" => Token::NormalLetter('ŋ'),
    "ngtr" => Token::Operator(ops::NOT_GREATER_THAN),
    "ni" => Token::Operator(ops::CONTAINS_AS_MEMBER),
    "nleftarrow" => Token::Operator(ops::LEFTWARDS_ARROW_WITH_STROKE),
    "nleftrightarrow" => Token::Operator(ops::LEFT_RIGHT_ARROW_WITH_STROKE),
    "nless" => Token::Operator(ops::NOT_LESS_THAN),
    "nmid" => Token::Operator(ops::DOES_NOT_DIVIDE),
    "not" => Token::Not,
    "notin" => Token::Operator(ops::NOT_AN_ELEMENT_OF),
    "nparallel" => Token::Operator(ops::NOT_PARALLEL_TO),
    "nprec" => Token::Operator(ops::DOES_NOT_PRECEDE),
    "nrightarrow" => Token::Operator(ops::RIGHTWARDS_ARROW_WITH_STROKE),
    "nsim" => Token::Operator(ops::NOT_TILDE),
    "nsubset" => Token::Operator(ops::NOT_A_SUBSET_OF),
    "nsubseteq" => Token::Operator(ops::NEITHER_A_SUBSET_OF_NOR_EQUAL_TO),
    "nsucc" => Token::Operator(ops::DOES_NOT_SUCCEED),
    "nsupset" => Token::Operator(ops::NOT_A_SUPERSET_OF),
    "nsupseteq" => Token::Operator(ops::NEITHER_A_SUPERSET_OF_NOR_EQUAL_TO),
    "nu" => Token::Letter('ν'),
    "nwarrow" => Token::Operator(ops::NORTH_WEST_ARROW),
    "o" => Token::NormalLetter('ø'),
    "odot" => Token::Operator(ops::CIRCLED_DOT_OPERATOR),
    "oe" => Token::NormalLetter('œ'),
    "oint" => Token::Integral(ops::CONTOUR_INTEGRAL),
    "omega" => Token::Letter('ω'),
    "omicron" => Token::Letter('ο'),
    "ominus" => Token::Operator(ops::CIRCLED_MINUS),
    "operatorname" => Token::OperatorName,
    "oplus" => Token::Operator(ops::CIRCLED_PLUS),
    "oslash" => Token::Operator(ops::CIRCLED_DIVISION_SLASH),
    "otimes" => Token::Operator(ops::CIRCLED_TIMES),
    "overbrace" => Token::Overbrace(ops::TOP_CURLY_BRACKET),
    "overbracket" => Token::Overbrace(ops::TOP_SQUARE_BRACKET),
    "overleftarrow" => Token::Over(ops::LEFTWARDS_ARROW),
    "overline" => Token::Over(ops::LOW_LINE),
    "overparen" => Token::Overbrace(ops::TOP_PARENTHESIS),
    "overrightarrow" => Token::Over(ops::RIGHTWARDS_ARROW),
    "overset" => Token::Overset,
    "parallel" => Token::Operator(ops::PARALLEL_TO),
    "partial" => Token::Letter('∂'),
    "perp" => Token::Operator(ops::UP_TACK),
    "phi" => Token::Letter('ϕ'),
    "pi" => Token::Letter('π'),
    "pm" => Token::Operator(ops::PLUS_MINUS_SIGN),
    "pounds" => Token::NormalLetter('£'),
    "prec" => Token::Operator(ops::PRECEDES),
    "preceq" => Token::Operator(ops::PRECEDES_ABOVE_SINGLE_LINE_EQUALS_SIGN),
    "prime" => Token::Operator(ops::PRIME),
    "prod" => Token::BigOp(ops::N_ARY_PRODUCT),
    "propto" => Token::Operator(ops::PROPORTIONAL_TO),
    "psi" => Token::Letter('ψ'),
    "qquad" => Token::Space("2"),
    "quad" => Token::Space("1"),
    "rangle" => Token::Paren(ops::MATHEMATICAL_RIGHT_ANGLE_BRACKET),
    "rceil" => Token::Paren(ops::RIGHT_CEILING),
    "rfloor" => Token::Paren(ops::RIGHT_FLOOR),
    "rgroup" => Token::Paren(ops::MATHEMATICAL_RIGHT_FLATTENED_PARENTHESIS),
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
    "rrbracket" => Token::Paren(ops::MATHEMATICAL_RIGHT_WHITE_SQUARE_BRACKET),
    "rtimes" => Token::Operator(ops::RIGHT_NORMAL_FACTOR_SEMIDIRECT_PRODUCT),
    "rupee" => Token::NormalLetter('₹'),
    "rvert" => Token::Paren(ops::VERTICAL_LINE),
    "saturn" => Token::NormalLetter('♄'),
    "scriptstyle" => Token::Style(Style::ScriptStyle),
    "scriptscriptstyle" => Token::Style(Style::ScriptScriptStyle),
    "searrow" => Token::Operator(ops::SOUTH_EAST_ARROW),
    "sec" => Token::Function("sec"),
    "setminus" => Token::Operator(ops::SET_MINUS),
    "sharp" => Token::NormalLetter('♯'),
    "sigma" => Token::Letter('σ'),
    "sim" => Token::Operator(ops::TILDE_OPERATOR),
    "simeq" => Token::Operator(ops::ASYMPTOTICALLY_EQUAL_TO),
    "sin" => Token::Function("sin"),
    "sinh" => Token::Function("sinh"),
    "slashed" => Token::Slashed,
    "smallsetminus" => Token::Operator(ops::SMALL_REVERSE_SOLIDUS),
    "smile" => Token::Operator(ops::SMILE),
    "spadesuit" => Token::NormalLetter('♠'),
    "sphericalangle" => Token::NormalLetter('∢'),
    "sqcap" => Token::Operator(ops::SQUARE_CAP),
    "sqcup" => Token::Operator(ops::SQUARE_CUP),
    "sqrt" => Token::Sqrt,
    "sqsubset" => Token::Operator(ops::SQUARE_IMAGE_OF),
    "sqsubseteq" => Token::Operator(ops::SQUARE_IMAGE_OF_OR_EQUAL_TO),
    "sqsupset" => Token::Operator(ops::SQUARE_ORIGINAL_OF),
    "sqsupseteq" => Token::Operator(ops::SQUARE_ORIGINAL_OF_OR_EQUAL_TO),
    "square" => Token::NormalLetter('□'),
    "ss" => Token::NormalLetter('ß'),
    "star" => Token::Operator(ops::STAR_OPERATOR),
    "subset" => Token::Operator(ops::SUBSET_OF),
    "subseteq" => Token::Operator(ops::SUBSET_OF_OR_EQUAL_TO),
    "subsetneq" => Token::Operator(ops::SUBSET_OF_WITH_NOT_EQUAL_TO),
    "succ" => Token::Operator(ops::SUCCEEDS),
    "succeq" => Token::Operator(ops::SUCCEEDS_ABOVE_SINGLE_LINE_EQUALS_SIGN),
    "sum" => Token::BigOp(ops::N_ARY_SUMMATION),
    "sun" => Token::NormalLetter('☼'),
    "sup" => Token::Lim("sup"),
    "supset" => Token::Operator(ops::SUPERSET_OF),
    "supseteq" => Token::Operator(ops::SUPERSET_OF_OR_EQUAL_TO),
    "supsetneq" => Token::Operator(ops::SUPERSET_OF_WITH_NOT_EQUAL_TO),
    "swarrow" => Token::Operator(ops::SOUTH_WEST_ARROW),
    "symbf" => Token::Transform(TextTransform::BoldItalic),
    "tan" => Token::Function("tan"),
    "tanh" => Token::Function("tanh"),
    "tau" => Token::Letter('τ'),
    "tbinom" => Token::Binom(Some(FracAttr::DisplayStyleFalse)),
    "text" => Token::Text,
    "textbf" => Token::Transform(TextTransform::Bold),
    "textit" => Token::Transform(TextTransform::Italic),
    "textstyle" => Token::Style(Style::TextStyle),
    "texttt" => Token::Transform(TextTransform::Monospace),
    "textyen" => Token::NormalLetter('¥'),
    "tfrac" => Token::Frac(Some(FracAttr::DisplayStyleFalse)),
    "th" => Token::NormalLetter('þ'),
    "therefore" => Token::NormalLetter('∴'),
    "theta" => Token::Letter('θ'),
    "tilde" => Token::Over(ops::TILDE),
    "times" => Token::Operator(ops::MULTIPLICATION_SIGN),
    "to" => Token::Operator(ops::RIGHTWARDS_ARROW),
    "top" => Token::Operator(ops::DOWN_TACK),
    "triangle" => Token::NormalLetter('△'),
    "triangleq" => Token::Operator(ops::DELTA_EQUAL_TO),
    "triangledown" => Token::Operator(ops::WHITE_DOWN_POINTING_TRIANGLE),
    "triangleleft" => Token::Operator(ops::WHITE_LEFT_POINTING_TRIANGLE),
    "triangleright" => Token::Operator(ops::WHITE_RIGHT_POINTING_TRIANGLE),
    "underbrace" => Token::Underbrace(ops::BOTTOM_CURLY_BRACKET),
    "underbracket" => Token::Underbrace(ops::BOTTOM_SQUARE_BRACKET),
    "underline" => Token::Under(ops::LOW_LINE),
    "underparen" => Token::Underbrace(ops::BOTTOM_PARENTHESIS),
    "underset" => Token::Underset,
    "unlhd" => Token::Operator(ops::NORMAL_SUBGROUP_OF_OR_EQUAL_TO),
    "unrhd" => Token::Operator(ops::CONTAINS_AS_NORMAL_SUBGROUP_OR_EQUAL_TO),
    "uparrow" => Token::Paren(ops::UPWARDS_ARROW),
    "updownarrow" => Token::Operator(ops::UP_DOWN_ARROW),
    "upharpoonleft" => Token::Operator(ops::UPWARDS_HARPOON_WITH_BARB_LEFTWARDS),
    "upharpoonright" => Token::Operator(ops::UPWARDS_HARPOON_WITH_BARB_RIGHTWARDS),
    "uplus" => Token::Operator(ops::MULTISET_UNION),
    "upsilon" => Token::Letter('υ'),
    "upuparrows" => Token::Operator(ops::UPWARDS_PAIRED_ARROWS),
    "uranus" => Token::NormalLetter('♅'),
    "vDash" => Token::Operator(ops::TRUE),
    "varepsilon" => Token::Letter('ε'),
    "varnothing" => Token::Letter('⌀'),
    "varphi" => Token::Letter('φ'),
    "varpi" => Token::Letter('ϖ'),
    "varrho" => Token::Letter('ϱ'),
    "varsigma" => Token::Letter('ς'),
    "vartheta" => Token::Letter('ϑ'),
    "vartriangle" => Token::Operator(ops::WHITE_UP_POINTING_TRIANGLE),
    "vdash" => Token::Operator(ops::RIGHT_TACK),
    "vdots" => Token::Operator(ops::VERTICAL_ELLIPSIS),
    "vec" => Token::Over(ops::RIGHTWARDS_ARROW),
    "vee" => Token::Operator(ops::LOGICAL_OR),
    "veebar" => Token::Operator(ops::XOR),
    "venus" => Token::NormalLetter('♀'),
    "vert" => Token::Paren(ops::VERTICAL_LINE),
    "wedge" => Token::Operator(ops::LOGICAL_AND),
    "widehat" => Token::Over(ops::CIRCUMFLEX_ACCENT),
    "widetilde" => Token::Over(ops::TILDE),
    "wp" => Token::Function("℘"),
    "wr" => Token::Operator(ops::WREATH_PRODUCT),
    "xi" => Token::Letter('ξ'),
    "zeta" => Token::Letter('ζ'),
    "{" => Token::Paren(ops::LEFT_CURLY_BRACKET),
    "|" => Token::Paren(ops::DOUBLE_VERTICAL_LINE),
    "}" => Token::Paren(ops::RIGHT_CURLY_BRACKET),
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
