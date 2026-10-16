# urng 1.0.0 修正・検証報告

2026-10-16。基点は `19bc224`。レビューのB1〜B21を対象に修正と回帰テストを追加し、以下の検証を完了した。**実装修正・回帰テスト・検証データの対応コミットは `b1573cc`**（`fix: correct RNG algorithms, range sampling and C ABI safety`）。下表の修正はすべてこのコミットに含まれる。

原文保存コミット: `d625c0f`。`REVIEW.md` は過去のassistant最終報告を会話記録から抽出したもので、UTF-8の31,660バイトが一致する。原文のMarkdown改行用末尾空白も維持した。

## 修正した問題とファイル

| 指摘 | 修正内容・主なファイル | 検出する回帰テスト |
|---|---|---|
| B1 | `src/prng/b64/xoshiro.rs`: Xoshiro256Ssx2の範囲外ストアを廃止。2個のxoshiro256**状態を更新して2語を返す | 2レーンをスカラーと独立した同時代入式に対して1024ステップ照合。初期出力も固定 |
| B2 | `src/cabi/**/*.rs`: fill/freeをunsafe化。`src/cabi/stream.rs`に所有ハンドル・端数バッファ・検査を共通化 | safe呼び出しが失敗するdoc test、null・不整列・過大count・不正範囲、全エクスポートの型照合 |
| B3 | `src/avx2.rs`、`src/prng/b32/{jsf,xoroshiro,xorshift}.rs`、b64のSIMDコンストラクター、`src/rngv.rs`: AVX2経路からAVX-512VL依存を除去。安全なコンストラクター/変換のCPU条件を処理 | AVX2/AVX-512の全レーン比較、target-cpu=nativeテスト。非対応CPUでの実機検証は未実施 |
| B4 | `src/crng/b32/{chacha,salsa}.rs`: 状態配置、feedforward、counter、ROUNDS/2回のダブルラウンド、Salsaのrotate 13を修正。`from_key_nonce`追加 | ChaCha20 RFC 8439、Salsa8 RFC 7914の既知出力。counter増分・桁上がり・枯渇後のpanic |
| B5 | `src/cabi/stream.rs`と全C ABI: チャンクごとの再シードを廃止し、一つの連続系列を消費 | SFC64x8/Biski64x8/Cet64x8で旧チャンク境界を越える8193/32769/131073語を比較 |
| B6 | `src/wide/splitmix.rs`、`src/prng/b32/splitmix.rs`: 各レーンを幅×増分で進め、スカラーと同じfinalizerに統一 | wideの4/8/16幅とSIMD16幅、複数seed・複数ブロックの全出力比較 |
| B7 | C ABIのPhiloxを共有バッファへ移行。端数でcounterを過剰に進めない | 全自然アラインメント、短い長さ、分割呼び出し、書き込み前後のcanary、次の出力まで比較 |
| B8 | `src/prng/b64/xorshift.rs`: シード展開でゼロになった状態を1へ補正 | seed `0x61c8864680b583eb`を100ステップ独立式と比較 |
| B9 | `src/prng/b32/pcg.rs`、`src/wide/pcg.rs`: incを全レーンで奇数化 | PCG公式デモの6語、1024seedの奇数inc、SIMD8/wide4・8・16の全レーン参照比較 |
| B10 | `src/prng/b64/xoroshiro.rs`: ++遷移と両形式のlong_jumpを修正 | ++/**の独立参照式1024ステップ、jump/long_jump後128語を複数seedで比較 |
| B11 | `src/cbrng/b32/philox.rs`: Random123 Philox4x32-10に修正。SIMDも同じ正しいblock計算を使用 | 公式ゼロKAT、独立参照式、4系列・16語すべて、counter桁上がり |
| B12 | `src/prng/b64/sfc.rs`: cを旧cのrotateと結果の和で更新 | スカラーの独立式、SIMD8レーンをスカラー/参照式と1024ステップ比較 |
| B13 | `src/prng/b64/biski.rs`、`src/prng/b32/xoroshiro.rs`: 旧状態の参照、**出力のrotate/multiplyを修正 | Biski8、xoroshiro8/16の全レーンをスカラー/独立式と比較 |
| B14 | `src/cbrng/b64/{philox,threefish}.rs`: 初回posを空バッファの位置へ変更 | 最初の4語を実際に生成したblockと比較。Philox2x64/4x64にはRandom123 KATも追加 |
| B15 | `src/prng/b64/twisted_gfsr.rs`: 32-bit coreの2語を結合しWord=u64の幅に合わせる | 上位32bitの非ゼロ、choiceで各添字へ到達することを検査 |
| B16 | `src/rngv.rs`: 各レーン固有の入力・scaleでmultiply-highを計算 | 128/256/512-bitの全レーンに異なる値・scaleを与えて独立計算と比較 |
| B17 | `src/rng.rs`、`src/wide/wrng.rs`、`src/rngv.rs`、明示的なSIMD rangeメソッド: 広い型で符号付き加算。乱数を引けるAPIではrejection | i32/i64の全範囲・跨ゼロ範囲、棄却を強制する入力、SIMD/wideのレーンごとの棄却 |
| B18 | 同上のmin/max float API: 幅のoverflowを避け、丸めでmaxになった場合はmax.next_downへ制限 | 隣接float、subnormal、±MAX、最大乱数語。C ABIのfloat経路も照合 |
| B19 | `src/shuffle.rs`、`src/choice.rs`: unbiased indexと縮小範囲のFisher–Yates | 3要素の6通りを等確率分岐から全列挙。in-placeとiterator一致、空/1要素も確認 |
| B20 | `src/sampler/b32/bst.rs`、`src/sampler/b64/bst.rs`: rより厳密に大きい累積値を選ぶ。有限・非負・正の総和を要求 | 先頭/中間/末尾のゼロ重み、r=0/累積境界、無効な重み |
| B21 | `src/lib.rs`: include_strを実在するREADME.mdの綴りに修正 | `tests/packaging.rs`でディレクトリエントリーとの大文字小文字一致を確認 |

補足:

- U1の専用NTストア経路はC ABIから除去した。元の未検証の懸念を「再現済みUB」とは扱わない。
- U2は連続系列を仕様として明文化し、呼び出し分割・出力アドレス・数値型の混在でも端数を保持する。
- SplitMixの`seed | 1`を廃止し、偶数seedの下位ビットを保存。これを使う生成器の出力も変わる。
- 通常ビルドの確認中にC#生成で54関数が欠落していることを確認。`build.rs`の入力ファイル、生成先ディレクトリ作成、再実行条件を補修し、320関数すべてを照合した。
- 既存nativeテストのメソッド名/trait import不備を修正。テストは削除・無効化していない。choiceの固定期待値1件はシード修正に合わせ3から7へ更新した。

## テストとビルド結果

検証環境: Windows x86_64、rustc 1.98.1。AVX2・AVX-512F/DQ/VL対応ホスト。ソース固定後のfeature検証用hashは [validation/source.sha256](validation/source.sha256)。

| 実行内容 | 結果・保存ログ |
|---|---|
| `cargo test --locked --offline --all-features --all-targets` | **190成功**: 単体114、algorithms 8、C ABI59、packaging 1、ranges 6、SIMD変換2。[ログ](validation/all-target-tests.log) |
| 同コマンド、`RUSTFLAGS=-C target-cpu=native` | **199成功**、既存のCPU条件付きテスト9件を追加実行。[ログ](validation/native-final.log) |
| `cargo test --locked --offline --release --all-features --lib --tests` | **190成功**。[ログ](validation/release-tests.log) |
| `cargo test --locked --offline --all-features --doc` | **105成功**: 通常104 + compile_fail 1。[ログ](validation/doc-tests.log) |
| `cargo test --locked --offline --no-default-features` | **65テスト + 44 doc test成功**。未使用import/関数の警告は残る。[ログ](validation/default-tests.log) |
| `cargo check --locked --offline --no-default-features --all-targets` | 成功。[ログ](validation/default-check.log) |
| `python tools/check_features.py` | **128/128のlibビルド成功**。rand/cabi/wide/simd/sampler/seedgenと暗黙のthiserrorの全組み合わせ。fullはall-featuresで検証。[一覧](validation/features.csv) |
| `cargo build --locked --offline --no-default-features --features full` | full単独のエイリアス構成も成功。[ログ](validation/full-alias-build.log) |
| DOCS_RSを設定しない通常のall-features build | 成功、C#生成も実施。[ログ](validation/normal-build.log) |
| `python tools/check_exports.py target/normal/debug/urng.dll` | **320/320一致**: シンボル、引数・戻り値、Rustのunsafe境界、C#宣言。[ログ](validation/exports.log) |

上のCargoテストとfeature列挙では`DOCS_RS=1`を使用し、build.rsの文書/C#書き込みを分離した。別途通常ビルドを実施済み。examplesはall-targetsでコンパイルしたが、既存の大規模ベンチ用mainは実行していない。

C ABIの59件は、56ファミリーの公開された整数・float・範囲生成を含む。各自然アラインメント、0/1/2/3/15/16/17/63/64/65/257語、分割要求、前後のcanaryを検証し、長い旧チャンク境界、拒否される入力、数値型混在を別テストで確認した。

公式/一次資料は [RFC 8439 §2.3.2](https://www.rfc-editor.org/rfc/rfc8439.html#section-2.3.2)、[RFC 7914 §8](https://www.rfc-editor.org/rfc/rfc7914.html#section-8)、[PCG公式デモ](https://www.pcg-random.org/using-pcg-c-basic.html)、[Random123 KAT](https://github.com/DEShawResearch/random123/blob/main/tests/kat_vectors)を使用した。独立した遷移式と参照先は`tests/support/reference.rs`に保存。SIMDとの比較だけでスカラーを正解とはしていない。

## 互換性への影響

- 指摘対象のアルゴリズム、シード、float境界処理、範囲生成、shuffle、C ABIの出力系列は旧1.0.0から変わる。旧系列の再現より正しさを優先した。Cargo.tomlのversionは1.0.0のまま。
- RustからC ABIのfill/freeを呼ぶ場合、unsafeブロックが必要。Cのシンボルと型は維持した。ハンドルは追加のpending bufferを所有し、対応new/free専用のopaque handleとなる。旧ライブラリで作ったハンドルを新版に渡したり、RustのBox<Rng>で代用したりしてはならない。
- unsupported CPUのC SIMDコンストラクターはnull。安全なRustコンストラクターの一部はCPU非対応時にpanicする。unsafeなtarget-featureメソッドのCPU確認は引き続き呼び出し側の契約。
- integer rangeはmin≤max、float rangeは有限かつmin<maxを要求。Rustの不正入力はpanic、C ABIはno-op。rejectionにより消費する乱数語数は可変になる。RngVの整数scale=0は2^32幅を表す。
- `Word::to_randi/to_index`と`WordV::to_randi`は単語しか持たないため、偏りのある低水準変換として残す。unbiasedな乱数抽出にはRng/RngW/RngVの抽出APIを使う。`WordV::to_randf`/`RngV::randfv`もscaleを受け取る低水準affine変換として残り、min/max APIの半開区間保証とは区別する。
- ChaCha/Salsaの`new(u32)`とDefaultは暗号用途の十分なエントロピー源ではない。鍵/nonceによる構築を追加したが、暗号セキュリティ全体を監査したという意味ではない。

## 同一条件での性能比較

基点`19bc224`から修正前rlibを修正開始前に構築して保存し、最終版のrlibと比較。双方に同一の [tools/review_bench.rs](tools/review_bench.rs) を同じrustcフラグでリンクした。ライブラリはrelease/all-features、fat LTO、codegen-units=1。ハーネスはopt-level=3/target-cpu=native/panic=abort。Rayonは4スレッド。各処理1,048,576語、warmup 1回、計測9回、3組をbefore→after / after→before / before→afterの順で実行した。ビルド終了後に測定した。

以下は各プロセスの中央値3個の中央値。単位はns/語、比は修正後/修正前であり、大きいほど遅い。

| 処理 | 修正前 | 修正後 | 比 |
|---|---:|---:|---:|
| PCG32 raw | 0.7656 | 0.7661 | 1.00 |
| Philox32 raw | 3.7162 | 1.5429 | 0.42 |
| ChaCha20 raw | 14.4171 | 7.1106 | 0.49 |
| SFC64 raw | 0.4144 | 0.4289 | 1.03 |
| Biski64 raw | 0.3900 | 0.3912 | 1.00 |
| Xoshiro256++ raw | 0.5591 | 0.5607 | 1.00 |
| Xoroshiro128++ raw | 0.7674 | 0.6202 | 0.81 |
| PCG32 randi | 0.8635 | 3.0697 | 3.55 |
| SFC64 randi | 1.0576 | 2.9129 | 2.75 |
| SFC64 randf | 0.6842 | 1.8804 | 2.75 |
| wide SplitMix32x4 fill | 0.1276 | 0.2434 | 1.91 |
| C ABI PCG32 fill | 0.2399 | 0.7849 | 3.27 |
| C ABI SFC64x8 fill | 0.0335 | 0.8034 | 23.98 |

範囲生成のrejection・検査、float境界保証、wideのfinalizer統一、C ABIの端数保持と並列再シード廃止にコストがある。特に最後の約24倍の増加は大きい。旧実装の系列重複を再導入して速度を維持することはしなかった。今後は同じ連続系列を維持したblockコピーや妥当性を証明したjump-aheadを検討できるが、未実装・未測定である。

Philox/ChaChaの短縮には誤ったラウンド数の修正も含まれ、同一アルゴリズムの単純な最適化とは解釈しない。CPU固定/周波数制御はしておらず、数%の差に有意性を主張しない。全生成器・全サイズの性能評価ではない。B1の旧実装は範囲外ストアがあるため測定対象にしなかった。

[全6回のCSV・条件](validation/benchmark/metadata.json)、[集計CSV](validation/benchmark/comparison.csv)、再実行用 [tools/compare_bench.py](tools/compare_bench.py) を保存した。

## 修正できなかった範囲・残存リスク

- B1〜B21の表に示した欠陥修正は実装済み。ただし未実施の検証を含む完全な安全性証明ではない。
- Miriはインストールされておらず未実行。ASan/TSan、非対応CPU、AVX2のみのCPU、ARM、32-bit、Linuxの大小文字区別ファイルシステム、MSRV 1.92は未検証。B21は実ファイル名のcase一致をテストした範囲。
- C ABIはポインターの寿命、所有権、実際の確保サイズ、排他的アクセスを完全には動的検査できない。callerのunsafe契約が必要。意図的なuse-after-freeや短すぎる実allocationを実行して安全性を主張してはいない。
- 公式KATは上記の対象に限定。Salsa20への直接の20-round KAT、Threefishの暗号全体の公式ベクトル、残り全生成器の独立した公式照合は未実施。Threefishは今回の初回buffer欠陥を検証した。SIMDの独立参照/スカラー全レーン比較は表の修正対象に対して実施し、未変更の全SIMDアルゴリズムを新規監査したわけではない。
- TestU01/PractRandなど長期統計試験、全seedの探索、周期/系列独立性の証明は未実施。TwistedGFSRの既存シード処理は下位32bitのみを使うため、上位bitだけ異なるseedは衝突することを文書化して残した。
- レビューCの提案であるrand_coreの64-bit API拡充、seed byte order、no_std化、既存benchmark一式の公平化は今回全面対応していない。比較には専用の共通ハーネスを用いた。
- C#は宣言の存在と型の元となるRustエクスポートを照合した範囲で、C#ランタイムからの実行は未検証。Rustからのextern C呼び出しと実DLLシンボルを検証した。

## 作業・引き継ぎ記録

実装と要求されたローカル検証は完了。未検証範囲は上記のとおり。検証ソースhash、ログ、CSVは`validation/`、再現用コードは`tests/`と`tools/`に保存した。以下は作業途中の記録であり、途中の「未完」は現在の状態を表さない。

<details>
<summary>途中経過（履歴）</summary>

基点: `19bc224` (`phase 1: review`)。開始日: 2026-10-16。
レビュー原文は会話記録から抽出し、`REVIEW.md` にUTF-8でそのまま保存した。

## 方針と優先順位

1. P0: B1–B3 / U1 — SIMDのメモリ安全性、CPU機能条件、C ABIのunsafe境界とストア。
2. P1: B4–B15 — 標準アルゴリズム、シード、並列系列、ブロック管理。
3. P2: B16–B21 — 数値変換、shuffle、sampler、ビルド。
4. 全体検証: 単体・回帰・doc test、feature組み合わせ、同一ハーネスでの修正前後ベンチマーク。

各段階でこのファイルを更新する。未検証・未完了の項目は完了として扱わない。
安全性のためのAPI変更と、性能を落として正しさを確保する変更はここに記録する。

## 進捗

| 対象 | 状態 | 対応・検証 |
|---|---|---|
| REVIEW.md原文保存 | 完了 | 会話記録のassistant最終出力とUTF-8バイト一致 |
| 修正前ベンチマーク | 測定済み | `tools/review_bench.rs`、`target/review/before_bench.csv` |
| B1–B3 / U1 | 実装・関連検証済み | B1参照式1024ステップ、C ABI 57テスト、SIMD変換1テスト。非対応CPUでの実機検証は未実施 |
| B4–B15 | 修正・関連検証済み | RFC/PCG/Random123既知出力、独立参照式、SIMD全レーン。最終全体検証は後続 |
| B16–B21 | 修正・関連検証中 | WordV、整数rejection、浮動小数境界、Fisher–Yates、累積重み、README大文字小文字 |
| U2/API仕様 | 修正済み | C ABIの端数を保持し、呼び出し分割とアドレスに依存しない系列へ |
| 全体検証 | 最終確認中 | 全feature/all-targets 190件、doc 105件成功。128構成の初回build成功、最終ソース固定後の再build中 |
| 修正後ベンチマーク | ビルド済み・測定待ち | `target/after/release/liburng.rlib`。feature build終了後に同じハーネスで測定 |

## 引き継ぎ

- 作業ディレクトリ: `D:\.repo\urng-review_gpt-6-astra`。
- 現環境: Windows x86_64、rustc 1.98.1。レビュー時の再現コードは `target/review/probes.rs`。
- 既存の再現コードは「欠陥の存在」をassertするため、そのまま修正後の合否判定には使わない。
- Cargo検証では当面 `DOCS_RS=1` を指定し、build.rsによるREADME/C#生成を分離する。
- 修正・回帰テスト・性能測定はまだ完了していない。

## 互換性・性能・残存リスク

後続の各段階で実測・確認した内容を追記する。

## 段階1: メモリ安全性とC ABI

- B1: `Xoshiro256Ssx2`は2組のxoshiro256**状態をスカラー語で正しく更新する。戻り値2語へ通常の代入を行い、AVX-512不要。公開型名・戻り値・状態のサイズは維持。
- B2: 全320個の既存Cエクスポートを棚卸し。free/fillをunsafe化し、所有権とバッファの契約を記載。Rust呼び出し元にはunsafeブロックが必要。
- B3: AVX2経路のAVX-512VL専用命令をAVX2演算へ変更。機能検出のない安全なSIMDコンストラクターにチェックを追加。安全なWordV変換は配列を介してCPU機能に依存しない実装へ変更。
- B5/B7/U1/U2: C ABI専用の複製アルゴリズム・再シード・NTストアを、各生成器を使う逐次のbuffered streamへ置換。端数をopaque handleに保持し、アドレスと要求の分割に依存しない系列にする。
- C ABIの型付きポインターとシンボル名は維持。ハンドルは追加バッファを所有するため、必ず対応するnew/freeで管理する。既存のポインターを新版へ持ち越すことは不可。
- 性能低下を予想: 並列再シードとILP専用カーネルを外したため、C ABIの大量生成は低下し得る。正しい系列と端数管理を優先。最終段階で同一条件の実測を記載する。
- 関連検証: B1専用単体1件、C ABI回帰57件、SIMD変換1件が通過。全feature/all-targetsのcheck成功。生成コードの不要unsafe/未使用警告は後続で整理する。
- 次: 参照アルゴリズム修正（B4/B6/B8–B15）、数値・分布API（B17–B20）、全体検証。

## 段階2: アルゴリズムと範囲変換

- ChaCha/Salsaを鍵・nonce・counter・feedforward・正しいラウンド数で実装。`from_key_nonce`を追加。最終カウンタのブロック消費後はpanicし、系列を繰り返さない。
- Philox4x32-10、SFC64、xoroshiro128++とlong_jump、Biski64 SIMD、xoroshiro64** SIMDを修正。
- SplitMixのseed低ビットを保存し、wide/SIMDの全出力を連続するスカラー系列へ統一。PCGのincを全レーンで奇数化。Xorshift64のゼロ状態を回避。
- Philox4x64/Threefish256の初回バッファ位置を修正。TwistedGFSRは32ビット出力2語からWord=u64を構成。
- `Rng::randi`と`Rng::index`はrejectionを使用。`Word::to_randi/to_index`は単語変換APIを維持し、単独では偏りがあることを明記。符号付き加算は広い整数型で計算。
- 有限のfloat範囲のoverflowと上限への丸めを処理。shuffleは縮小する範囲のFisher–Yatesへ変更。Bstは厳密に大きい累積値を選択し、無効な重みを拒否。
- 段階検証: 単体112件、アルゴリズム結合8件、C ABI57件が通過。範囲変換テストと最終全体検証は別ログに記録する。
- 既存choiceテストのseed=0固定期待値を3から7へ更新。シード下位ビットの保存による仕様変更であり、テストは削除・無効化していない。
- 新規回帰の初回実行では、テストコードの所有権/macro構文エラーと、f64既存変換に対する境界入力の誤りを修正した。失敗ログも`target/review`に残す。
- 未完: API文書整理、feature構成ビルド、doc test、通常build.rs経路、後測定、コミット。

## 段階3: 全体検証（最終確認中）

- 最新のall-features/all-targets: 単体114 + algorithms 8 + C ABI 59 + packaging 1 + ranges 6 + SIMD変換2 = 190件成功。examplesはコンパイル確認（実行テスト0件）。`target/review/all-target-tests.log`。
- doc testは通常104 + unsafe境界のcompile_fail 1 = 105件成功。`target/review/doc-tests.log`。
- target-cpu=nativeの既存テストで発覚したメソッド名/trait importの不備を修正。既存テストは維持し、CPU対応時に実行する。途中段階で121件成功、最終ソースのnative検証は後続。
- 全7 feature（明示的な6種類と暗黙のthiserror）の128組み合わせで初回build成功。最終ソースを固定して再実行中。`tools/check_features.py`は前後のソースhash一致も検査する。
- `DOCS_RS`を設定しない通常ビルドも成功。C#生成に54宣言の欠落を発見し、入力ファイルと生成先ディレクトリ作成・再実行条件を補修。`tools/check_exports.py`で320件の宣言・引数・戻り値・unsafe境界と実DLLシンボルを照合済み。
- 安定版/インストール済みnightlyのいずれにもMiriは未導入。Miri/ASan、AVX2のみのCPU、非x86_64 OS、MSRV 1.92での検証は未実施。
- 最終ソース変更はここでいったん固定。残りはfeature build完了、native/release/defaultテスト、ベンチマーク、結果の保存、コミット。

</details>
