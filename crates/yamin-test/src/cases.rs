//! The 40-case catalog. Expected forms follow docs/spec.md.
//!
//! `runs` lists the expected *visual* runs from left to right. Each run's text
//! is in logical order; `Rtl` runs are shaped, reversed and mirrored on display
//! (see `display::render_run`). Rule ids cite UAX#9 (Pn, Wn, Nn, In, Ln, HLn)
//! and the yamin decisions in docs/spec.md section 7 (Dn).

use Run::{Ltr, Rtl};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Category {
    Arabic,
    Mixed,
    Digits,
    Paths,
    Tashkeel,
    LamAlef,
    Mirroring,
    Code,
}

impl Category {
    pub const ALL: [Category; 8] = [
        Category::Arabic,
        Category::Mixed,
        Category::Digits,
        Category::Paths,
        Category::Tashkeel,
        Category::LamAlef,
        Category::Mirroring,
        Category::Code,
    ];

    pub fn slug(self) -> &'static str {
        match self {
            Category::Arabic => "arabic",
            Category::Mixed => "mixed",
            Category::Digits => "digits",
            Category::Paths => "paths",
            Category::Tashkeel => "tashkeel",
            Category::LamAlef => "lamalef",
            Category::Mirroring => "mirroring",
            Category::Code => "code",
        }
    }

    pub fn from_slug(s: &str) -> Option<Category> {
        Category::ALL.into_iter().find(|c| c.slug() == s)
    }

    pub fn title(self) -> &'static str {
        match self {
            Category::Arabic => "عربي صافٍ (pure Arabic)",
            Category::Mixed => "عربي + إنجليزي (mixed)",
            Category::Digits => "أرقام (digits)",
            Category::Paths => "مسارات ملفات (paths)",
            Category::Tashkeel => "تشكيل (harakat)",
            Category::LamAlef => "لام-ألف (lam-alef)",
            Category::Mirroring => "أقواس معكوسة (mirroring)",
            Category::Code => "كود داخل جملة (code)",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Ltr,
    Rtl,
}

impl Dir {
    pub fn as_str(self) -> &'static str {
        match self {
            Dir::Ltr => "LTR",
            Dir::Rtl => "RTL",
        }
    }
}

/// A run in *visual* left-to-right order. The text is stored in logical order;
/// `Rtl` runs are shaped, reversed by cluster, and mirrored when displayed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Run {
    Ltr(&'static str),
    Rtl(&'static str),
}

#[derive(Debug)]
pub struct Case {
    pub id: &'static str,
    pub category: Category,
    /// Paragraph direction, set explicitly (D5: autodetection off).
    pub dir: Dir,
    /// Logical-order input, exactly as an application would print it.
    pub input: &'static str,
    pub runs: &'static [Run],
    pub description: &'static str,
    pub rules: &'static [&'static str],
}

pub fn all() -> &'static [Case] {
    CASES
}

pub fn find(id: &str) -> Option<&'static Case> {
    CASES.iter().find(|c| c.id == id)
}

use Category as C;

static CASES: &[Case] = &[
    // ── عربي صافٍ ─────────────────────────────────────────────
    Case {
        id: "arabic-01",
        category: C::Arabic,
        dir: Dir::Rtl,
        input: "مرحبا بالعالم",
        runs: &[Rtl("مرحبا بالعالم")],
        description: "جملة عربية كاملة: الحروف متصلة، والكلمة الأولى على اليمين، والسطر محاذى لليمين.",
        rules: &["HL1", "L2", "shaping"],
    },
    Case {
        id: "arabic-02",
        category: C::Arabic,
        dir: Dir::Rtl,
        input: "كيف حالك؟",
        runs: &[Rtl("كيف حالك؟")],
        description: "علامة الاستفهام العربية (AL) تظهر في أقصى اليسار بعد آخر كلمة.",
        rules: &["HL1", "L2", "shaping"],
    },
    Case {
        id: "arabic-03",
        category: C::Arabic,
        dir: Dir::Ltr,
        input: "مرحبا",
        runs: &[Rtl("مرحبا")],
        description: "فقرة LTR (D5: الكشف التلقائي معطّل): الكلمة معكوسة ومتصلة لكن السطر محاذى لليسار.",
        rules: &["D5", "I1", "L2", "shaping"],
    },
    Case {
        id: "arabic-04",
        category: C::Arabic,
        dir: Dir::Rtl,
        input: "شمس وقمر جميل",
        runs: &[Rtl("شمس وقمر جميل")],
        description: "أشكال أول/وسط/آخر/منفرد: و تقطع الاتصال فتبدأ ق بشكلها الأول.",
        rules: &["HL1", "L2", "shaping"],
    },
    Case {
        id: "arabic-05",
        category: C::Arabic,
        dir: Dir::Rtl,
        input: "نعم، شكرا.",
        runs: &[Rtl("نعم، شكرا.")],
        description: "الفاصلة العربية والنقطة الأخيرة تأخذان اتجاه الفقرة؛ النقطة في أقصى اليسار.",
        rules: &["W6", "N1", "N2", "L2"],
    },
    // ── عربي + إنجليزي ─────────────────────────────────────────
    Case {
        id: "mixed-01",
        category: C::Mixed,
        dir: Dir::Rtl,
        input: "مرحبا world",
        runs: &[Ltr("world"), Rtl("مرحبا ")],
        description: "الكلمة الإنجليزية على اليسار وتُقرأ من اليسار لليمين، والعربية على اليمين.",
        rules: &["HL1", "N2", "I2", "L2"],
    },
    Case {
        id: "mixed-02",
        category: C::Mixed,
        dir: Dir::Ltr,
        input: "hello مرحبا",
        runs: &[Ltr("hello "), Rtl("مرحبا")],
        description: "فقرة LTR: الإنجليزية أولًا من اليسار ثم العربية معكوسة ومتصلة.",
        rules: &["N2", "I1", "L2"],
    },
    Case {
        id: "mixed-03",
        category: C::Mixed,
        dir: Dir::Rtl,
        input: "أنا أستخدم Rust يوميا",
        runs: &[Rtl(" يوميا"), Ltr("Rust"), Rtl("أنا أستخدم ")],
        description: "كلمة إنجليزية في وسط جملة عربية: تبقى في مكانها المنطقي وتُقرأ LTR.",
        rules: &["N1", "N2", "I2", "L2"],
    },
    Case {
        id: "mixed-04",
        category: C::Mixed,
        dir: Dir::Rtl,
        input: "استخدم git commit فورا",
        runs: &[Rtl(" فورا"), Ltr("git commit"), Rtl("استخدم ")],
        description: "كلمتان إنجليزيتان متتاليتان تبقيان بنفس الترتيب (git ثم commit) لأن المسافة بينهما L.",
        rules: &["N1", "I2", "L2"],
    },
    Case {
        id: "mixed-05",
        category: C::Mixed,
        dir: Dir::Rtl,
        input: "مرحبا world!",
        runs: &[Rtl("!"), Ltr("world"), Rtl("مرحبا ")],
        description: "الحالة الكلاسيكية: علامة التعجب بعد الكلمة الإنجليزية تظهر على يسارها (!world) لأنها تأخذ اتجاه الفقرة.",
        rules: &["N1", "N2", "L2"],
    },
    // ── أرقام ─────────────────────────────────────────────────
    Case {
        id: "digits-01",
        category: C::Digits,
        dir: Dir::Rtl,
        input: "العدد ١٢٣",
        runs: &[Ltr("١٢٣"), Rtl("العدد ")],
        description: "أرقام عربية-هندية (AN): الرقم على اليسار وخاناته تُقرأ من اليسار لليمين (١٢٣).",
        rules: &["AN", "I2", "L2"],
    },
    Case {
        id: "digits-02",
        category: C::Digits,
        dir: Dir::Rtl,
        input: "السعر 25 ريال",
        runs: &[Rtl(" ريال"), Ltr("25"), Rtl("السعر ")],
        description: "رقم لاتيني بعد عربي يصبح AN (W2)، ويبقى 25 لا 52.",
        rules: &["W2", "I2", "L2"],
    },
    Case {
        id: "digits-03",
        category: C::Digits,
        dir: Dir::Rtl,
        input: "النسبة ٣٫١٤",
        runs: &[Ltr("٣٫١٤"), Rtl("النسبة ")],
        description: "الفاصلة العشرية العربية (٫ من نوع AN) تُبقي الرقم وحدة واحدة: ٣٫١٤.",
        rules: &["AN", "W4", "I2"],
    },
    Case {
        id: "digits-04",
        category: C::Digits,
        dir: Dir::Rtl,
        input: "الناتج 5-3",
        runs: &[Ltr("3"), Rtl("-"), Ltr("5"), Rtl("الناتج ")],
        description: "بعد العربي يصبح 5 و3 من نوع AN، فلا يضمهما W4 (يخص EN). الشرطة تأخذ اتجاه R فيظهر 3-5.",
        rules: &["W2", "W4", "W6", "N1", "L2"],
    },
    Case {
        id: "digits-05",
        category: C::Digits,
        dir: Dir::Rtl,
        input: "خصم 50%",
        runs: &[Rtl("%"), Ltr("50"), Rtl("خصم ")],
        description: "بعد العربي يصبح 50 من نوع AN فلا يضم W5 علامة %؛ تظهر % على يسار الرقم (%50).",
        rules: &["W2", "W5", "W6", "N1", "L2"],
    },
    // ── مسارات ─────────────────────────────────────────────────
    Case {
        id: "paths-01",
        category: C::Paths,
        dir: Dir::Rtl,
        input: "افتح الملف /usr/bin/env",
        runs: &[Ltr("usr/bin/env"), Rtl("افتح الملف /")],
        description: "الشرطة المائلة الأولى بين عربي ولاتيني تأخذ اتجاه الفقرة، فتظهر بعد المسار: usr/bin/env/ (صحيح حسب UAX#9 لكنه مربك — HL4).",
        rules: &["W6", "N1", "N2", "L2", "HL4"],
    },
    Case {
        id: "paths-02",
        category: C::Paths,
        dir: Dir::Rtl,
        input: "انتقل إلى src/main/",
        runs: &[Rtl("/"), Ltr("src/main"), Rtl("انتقل إلى ")],
        description: "الشرطة المائلة الأخيرة في نهاية سطر RTL تنتقل إلى يسار المسار: /src/main.",
        rules: &["W6", "N1", "N2", "L2"],
    },
    Case {
        id: "paths-03",
        category: C::Paths,
        dir: Dir::Rtl,
        input: "المسار C:\\Users\\test",
        runs: &[Ltr("C:\\Users\\test"), Rtl("المسار ")],
        description: "مسار Windows محصور بين حروف لاتينية فيبقى سليمًا على اليسار.",
        rules: &["W6", "N1", "I2", "L2"],
    },
    Case {
        id: "paths-04",
        category: C::Paths,
        dir: Dir::Ltr,
        input: "/home/مستخدم/مستندات",
        runs: &[Ltr("/home/"), Rtl("مستخدم/مستندات")],
        description: "فقرة LTR بمجلدات عربية: المقطعان العربيان يتبادلان المكان بصريًا (مستندات يظهر قبل مستخدم من اليسار).",
        rules: &["N1", "N2", "I1", "L2"],
    },
    Case {
        id: "paths-05",
        category: C::Paths,
        dir: Dir::Rtl,
        input: "زر https://example.com/ar",
        runs: &[Ltr("https://example.com/ar"), Rtl("زر ")],
        description: "رابط URL كامل داخل جملة عربية يبقى سليمًا لأن فواصله محصورة بين حروف L.",
        rules: &["W6", "N1", "I2", "L2"],
    },
    // ── تشكيل ─────────────────────────────────────────────────
    Case {
        id: "tashkeel-01",
        category: C::Tashkeel,
        dir: Dir::Rtl,
        input: "مَرْحَبًا",
        runs: &[Rtl("مَرْحَبًا")],
        description: "التشكيل فوق حرفه ولا يأخذ خلية مستقلة؛ الاتصال لا يتأثر بالحركات.",
        rules: &["W1", "L3", "D4", "shaping"],
    },
    Case {
        id: "tashkeel-02",
        category: C::Tashkeel,
        dir: Dir::Rtl,
        input: "بِسْمِ اللَّهِ",
        runs: &[Rtl("بِسْمِ اللَّهِ")],
        description: "شدة + فتحة على حرف واحد (لَّ) تظهران معًا فوقه.",
        rules: &["W1", "L3", "D4", "shaping"],
    },
    Case {
        id: "tashkeel-03",
        category: C::Tashkeel,
        dir: Dir::Rtl,
        input: "كُتُبٌ",
        runs: &[Rtl("كُتُبٌ")],
        description: "ضمة وتنوين ضم: الكلمة ثلاث خلايا فقط.",
        rules: &["W1", "L3", "D4", "shaping"],
    },
    Case {
        id: "tashkeel-04",
        category: C::Tashkeel,
        dir: Dir::Ltr,
        input: "word: عِلْمٌ",
        runs: &[Ltr("word: "), Rtl("عِلْمٌ")],
        description: "كلمة مشكولة في فقرة LTR: الحركات تبقى على حروفها بعد العكس.",
        rules: &["W1", "L3", "N2", "D4"],
    },
    Case {
        id: "tashkeel-05",
        category: C::Tashkeel,
        dir: Dir::Rtl,
        input: "هٰذا",
        runs: &[Rtl("هٰذا")],
        description: "الألف الخنجرية (U+0670) فوق الهاء؛ ذ لا تتصل يسارًا فتظهر ا منفردة.",
        rules: &["W1", "L3", "D4", "shaping"],
    },
    // ── لام-ألف ────────────────────────────────────────────────
    Case {
        id: "lamalef-01",
        category: C::LamAlef,
        dir: Dir::Rtl,
        input: "لا",
        runs: &[Rtl("لا")],
        description: "لام-ألف منفردة (ﻻ) ممتدة على خليتين (D1).",
        rules: &["D1", "shaping"],
    },
    Case {
        id: "lamalef-02",
        category: C::LamAlef,
        dir: Dir::Rtl,
        input: "سلام",
        runs: &[Rtl("سلام")],
        description: "لام-ألف بشكلها الأخير (ﻼ) متصلة بالسين، والميم بعدها منفردة.",
        rules: &["D1", "shaping"],
    },
    Case {
        id: "lamalef-03",
        category: C::LamAlef,
        dir: Dir::Rtl,
        input: "لأن",
        runs: &[Rtl("لأن")],
        description: "لام + ألف بهمزة فوق (ﻷ)، والنون منفردة.",
        rules: &["D1", "shaping"],
    },
    Case {
        id: "lamalef-04",
        category: C::LamAlef,
        dir: Dir::Rtl,
        input: "الإسلام",
        runs: &[Rtl("الإسلام")],
        description: "لام-ألفان: (ﻹ) منفردة بعد ا، و(ﻼ) أخيرة بعد س.",
        rules: &["D1", "shaping"],
    },
    Case {
        id: "lamalef-05",
        category: C::LamAlef,
        dir: Dir::Rtl,
        input: "الآن",
        runs: &[Rtl("الآن")],
        description: "لام + ألف ممدودة (ﻵ).",
        rules: &["D1", "shaping"],
    },
    // ── أقواس معكوسة ───────────────────────────────────────────
    Case {
        id: "mirroring-01",
        category: C::Mirroring,
        dir: Dir::Rtl,
        input: "(مرحبا)",
        runs: &[Rtl("(مرحبا)")],
        description: "القوسان يُعكسان (L4) فيحيطان بالكلمة بشكل صحيح: (مرحبا).",
        rules: &["N0", "L4"],
    },
    Case {
        id: "mirroring-02",
        category: C::Mirroring,
        dir: Dir::Rtl,
        input: "النتيجة [نجاح]",
        runs: &[Rtl("النتيجة [نجاح]")],
        description: "أقواس مربعة في سياق RTL: [نجاح] تظهر سليمة على يسار الكلمة الأولى.",
        rules: &["N0", "L4"],
    },
    Case {
        id: "mirroring-03",
        category: C::Mirroring,
        dir: Dir::Rtl,
        input: "الدالة (main) تعمل",
        runs: &[Rtl(") تعمل"), Ltr("main"), Rtl("الدالة (")],
        description: "قوسان حول كلمة إنجليزية في جملة عربية يأخذان اتجاه R (N0) ويُعكسان، فيظهر (main) سليمًا.",
        rules: &["N0", "L4", "I2"],
    },
    Case {
        id: "mirroring-04",
        category: C::Mirroring,
        dir: Dir::Ltr,
        input: "call (دالة)",
        runs: &[Ltr("call ("), Rtl("دالة"), Ltr(")")],
        description: "في فقرة LTR القوسان L (N0) فلا يُعكسان، والكلمة العربية بينهما معكوسة.",
        rules: &["N0", "I1", "L2"],
    },
    Case {
        id: "mirroring-05",
        category: C::Mirroring,
        dir: Dir::Rtl,
        input: "قال «نعم»",
        runs: &[Rtl("قال «نعم»")],
        description: "علامتا التنصيص « » ليستا زوج أقواس (BidiBrackets) لكنهما Bidi_Mirrored فتُعكسان بقاعدة L4.",
        rules: &["N1", "L4"],
    },
    // ── كود داخل جملة ──────────────────────────────────────────
    Case {
        id: "code-01",
        category: C::Code,
        dir: Dir::Rtl,
        input: "شغّل cargo build --release فورا",
        runs: &[Rtl(" فورا"), Ltr("cargo build --release"), Rtl("شغّل ")],
        description: "أمر طرفي كامل داخل جملة عربية يبقى سليمًا، و--release في مكانها.",
        rules: &["W6", "N1", "I2", "L2"],
    },
    Case {
        id: "code-02",
        category: C::Code,
        dir: Dir::Rtl,
        input: "اكتب x = 5 ثم",
        runs: &[Rtl(" ثم"), Ltr("x = 5"), Rtl("اكتب ")],
        description: "الرقم بعد x يبقى EN ثم يصبح L (W7) فيبقى x = 5 بالترتيب.",
        rules: &["W2", "W7", "N1", "L2"],
    },
    Case {
        id: "code-03",
        category: C::Code,
        dir: Dir::Rtl,
        input: "استدعِ foo(1, 2) هنا",
        runs: &[Rtl(" هنا"), Ltr("foo(1, 2)"), Rtl("استدعِ ")],
        description: "استدعاء دالة بوسائط رقمية: الأقواس والأرقام L فيظهر foo(1, 2) سليمًا.",
        rules: &["W7", "N0", "N1", "L2"],
    },
    Case {
        id: "code-04",
        category: C::Code,
        dir: Dir::Rtl,
        input: "نفّذ main()",
        runs: &[Rtl("()"), Ltr("main"), Rtl("نفّذ ")],
        description: "() في نهاية السطر بلا محتوى قوي تأخذ اتجاه الفقرة فتظهر على اليسار: ()main.",
        rules: &["N0", "N1", "N2", "L4"],
    },
    Case {
        id: "code-05",
        category: C::Code,
        dir: Dir::Ltr,
        input: "let name = \"أحمد\";",
        runs: &[Ltr("let name = \""), Rtl("أحمد"), Ltr("\";")],
        description: "نص عربي داخل سلسلة في سطر كود LTR: علامتا التنصيص في مكانهما والاسم معكوس ومتصل.",
        rules: &["N2", "I1", "L2"],
    },
];
