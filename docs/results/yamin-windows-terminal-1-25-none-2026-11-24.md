# yamin-test report

| | |
|---|---|
| Terminal | Windows Terminal 1.25 |
| AI CLI | none |
| Date | 2026-11-24 |
| yamin-test | 0.1.0 |

**Result: 0 / 40 passed (0%)** — 40 failed, 0 skipped, 40 cases.

## By category

| Category | Pass | Fail | Skip |
|---|---|---|---|
| arabic | 0 | 5 | 0 |
| mixed | 0 | 5 | 0 |
| digits | 0 | 5 | 0 |
| paths | 0 | 5 | 0 |
| tashkeel | 0 | 5 | 0 |
| lamalef | 0 | 5 | 0 |
| mirroring | 0 | 5 | 0 |
| code | 0 | 5 | 0 |

## Failures

- **arabic-01** `مرحبا بالعالم` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **arabic-02** `كيف حالك؟` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **arabic-03** `مرحبا` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **arabic-04** `شمس وقمر جميل` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **arabic-05** `نعم، شكرا.` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **mixed-01** `مرحبا world` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **mixed-02** `hello مرحبا` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **mixed-03** `أنا أستخدم Rust يوميا` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **mixed-04** `استخدم git commit فورا` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **mixed-05** `مرحبا world!` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **digits-01** `العدد ١٢٣` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **digits-02** `السعر 25 ريال` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **digits-03** `النسبة ٣٫١٤` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **digits-04** `الناتج 5-3` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **digits-05** `خصم 50%` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **paths-01** `افتح الملف /usr/bin/env` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **paths-02** `انتقل إلى src/main/` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **paths-03** `المسار C:\Users\test` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **paths-04** `/home/مستخدم/مستندات` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **paths-05** `زر https://example.com/ar` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **tashkeel-01** `مَرْحَبًا` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **tashkeel-02** `بِسْمِ اللَّهِ` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **tashkeel-03** `كُتُبٌ` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **tashkeel-04** `word: عِلْمٌ` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **tashkeel-05** `هٰذا` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **lamalef-01** `لا` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **lamalef-02** `سلام` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **lamalef-03** `لأن` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **lamalef-04** `الإسلام` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **lamalef-05** `الآن` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **mirroring-01** `(مرحبا)` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **mirroring-02** `النتيجة [نجاح]` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **mirroring-03** `الدالة (main) تعمل` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **mirroring-04** `call (دالة)` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **mirroring-05** `قال «نعم»` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **code-01** `شغّل cargo build --release فورا` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **code-02** `اكتب x = 5 ثم` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **code-03** `استدعِ foo(1, 2) هنا` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **code-04** `نفّذ main()` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined
- **code-05** `let name = "أحمد";` — Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined

## All cases

| Case | Verdict | Note |
|---|---|---|
| arabic-01 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| arabic-02 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| arabic-03 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| arabic-04 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| arabic-05 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| mixed-01 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| mixed-02 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| mixed-03 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| mixed-04 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| mixed-05 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| digits-01 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| digits-02 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| digits-03 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| digits-04 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| digits-05 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| paths-01 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| paths-02 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| paths-03 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| paths-04 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| paths-05 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| tashkeel-01 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| tashkeel-02 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| tashkeel-03 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| tashkeel-04 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| tashkeel-05 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| lamalef-01 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| lamalef-02 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| lamalef-03 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| lamalef-04 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| lamalef-05 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| mirroring-01 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| mirroring-02 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| mirroring-03 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| mirroring-04 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| mirroring-05 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| code-01 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| code-02 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| code-03 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| code-04 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
| code-05 | ❌ fail | Claude judged from screenshots: no BiDi reordering or shaping; Arabic drawn in logical order left-to-right, letters unjoined |
