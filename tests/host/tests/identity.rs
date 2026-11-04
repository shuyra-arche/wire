//! 门面与规范包的身份夹具：`wire`（本仓）与 `kernel-api-wire` **是同一族 trait**。
//!
//! 每个探针只在**一侧**写下泛型上界，却在正文里用**另一侧**的同名手——两边不是同一枚
//! trait 的话这里编不过。两个方向各写一份：门面 ⇒ 规范、规范 ⇒ 门面。
//!
//! 枚举字节/UTF-8/截断那几条断言不在这里重复：它们仍由 `tests/host/lib.rs` 的五条既有
//! 夹具原样承担（本阶段一行都没改）。

use kernel_api_wire as canonical;
use wire as facade;

fn require_canonical_message<M: canonical::Message>() {}
fn require_canonical_contract<C: canonical::Contract>() {}
fn require_facade_contract<C: facade::Contract>() {}

// ── Field ────────────────────────────────────────────────────────────────

/// 只有门面上界，正文走规范包的 `WIDTH` 与 `fetch`。
fn facade_field_is_canonical<T: facade::Field>(bytes: &[u8]) -> Option<T> {
    assert_eq!(<T as facade::Field>::WIDTH, <T as canonical::Field>::WIDTH);
    <T as canonical::Field>::fetch(bytes)
}

/// 只有规范包上界，正文走门面的 `WIDTH` 与 `fetch`。
fn canonical_field_is_facade<T: canonical::Field>(bytes: &[u8]) -> Option<T> {
    assert_eq!(<T as canonical::Field>::WIDTH, <T as facade::Field>::WIDTH);
    <T as facade::Field>::fetch(bytes)
}

// ── Span ─────────────────────────────────────────────────────────────────

fn facade_span_is_canonical<T: facade::Span>(
    value: &T,
    out: &mut [u8],
    at: usize,
) -> Option<usize> {
    assert_eq!(<T as facade::Span>::MAX, <T as canonical::Span>::MAX);
    <T as canonical::Span>::store_at(value, out, at)
}

fn facade_span_fetch_is_canonical<T: facade::Span>(bytes: &[u8], at: usize) -> Option<(T, usize)> {
    <T as canonical::Span>::fetch_at(bytes, at)
}

fn canonical_span_is_facade<T: canonical::Span>(
    value: &T,
    out: &mut [u8],
    at: usize,
) -> Option<usize> {
    assert_eq!(<T as canonical::Span>::MAX, <T as facade::Span>::MAX);
    <T as facade::Span>::store_at(value, out, at)
}

// ── Message ──────────────────────────────────────────────────────────────

fn facade_message_is_canonical<M: facade::Message>(bytes: &[u8]) -> Option<M::In> {
    assert_eq!(<M as facade::Message>::MAX, <M as canonical::Message>::MAX);
    <M as canonical::Message>::fetch(bytes)
}

fn canonical_message_is_facade<M: canonical::Message>(bytes: &[u8]) -> Option<M::In> {
    <M as facade::Message>::fetch(bytes)
}

/// 关联类型 `Buf` 也是**同一枚**（不是两份同形的类型）：两个方向的转手都得编过。
fn message_buf_is_canonical<M: facade::Message>(buf: M::Buf) -> <M as canonical::Message>::Buf {
    buf
}

fn canonical_message_buf_is_facade<M: canonical::Message>(
    buf: M::Buf,
) -> <M as facade::Message>::Buf {
    buf
}

// ── Contract ─────────────────────────────────────────────────────────────

fn facade_contract_is_canonical<C: facade::Contract>() {
    require_canonical_contract::<C>();
    require_canonical_message::<<C as canonical::Contract>::Request>();
    require_canonical_message::<<C as canonical::Contract>::Response>();
}

fn canonical_contract_is_facade<C: canonical::Contract>() {
    require_facade_contract::<C>();
}

/// `Contract` 的 `Request`/`Response` 关联类型也是同一枚。
fn contract_request_is_same_type<C: facade::Contract>(
    request: <C as facade::Contract>::Request,
) -> <C as canonical::Contract>::Request {
    request
}

/// **只实现门面那一侧**的自定义包：随后要求它满足规范包上界。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Ping(u32);

impl facade::Message for Ping {
    type In = Ping;
    type Buf = [u8; 4];
    const EMPTY: Self::Buf = [0; 4];

    fn store(&self, out: &mut [u8]) -> Option<usize> {
        out.get_mut(..4)?.copy_from_slice(&self.0.to_le_bytes());
        Some(4)
    }

    fn fetch(bytes: &[u8]) -> Option<Self::In> {
        Some(Ping(u32::from_le_bytes(bytes.get(..4)?.try_into().ok()?)))
    }
}

impl facade::Contract for Ping {
    type Request = Ping;
    type Response = Ping;
}

#[test]
fn facade_and_canonical_field_span_bounds_transfer_both_ways() {
    let bytes: [u8; 4] = [0x78, 0x56, 0x34, 0x12];
    assert_eq!(facade_field_is_canonical::<u32>(&bytes), Some(0x1234_5678));
    assert_eq!(canonical_field_is_facade::<u32>(&bytes), Some(0x1234_5678));
    assert_eq!(facade_field_is_canonical::<u32>(&bytes[..3]), None);
    assert_eq!(canonical_field_is_facade::<u32>(&bytes[..3]), None);
    assert_eq!(facade_field_is_canonical::<bool>(&[2]), None);

    let name = String::from("猫é");
    let mut out = [0u8; 16];
    let end = facade_span_is_canonical(&name, &mut out, 0).unwrap();
    assert_eq!(&out[..end], &[5, 0xe7, 0x8c, 0xab, 0xc3, 0xa9]);
    assert_eq!(canonical_span_is_facade(&name, &mut out, 0), Some(end));
    assert_eq!(
        facade_span_fetch_is_canonical::<String>(&out[..end], 0),
        Some((name.clone(), end))
    );
    assert_eq!(
        facade_span_fetch_is_canonical::<String>(&out[..end - 1], 0),
        None
    );
}

#[test]
fn facade_only_packet_satisfies_canonical_bounds() {
    require_canonical_message::<Ping>();
    require_canonical_contract::<Ping>();
    require_canonical_message::<<Ping as canonical::Contract>::Request>();
    require_canonical_message::<<Ping as canonical::Contract>::Response>();

    let ping = Ping(0x1234_5678);
    assert_eq!(<Ping as canonical::Message>::MAX, 4);
    assert_eq!(<Ping as facade::Message>::MAX, 4);

    let mut buf = [0u8; 4];
    assert_eq!(
        <Ping as canonical::Message>::store(&ping, &mut buf),
        Some(4)
    );
    assert_eq!(buf, [0x78, 0x56, 0x34, 0x12]);
    assert_eq!(<Ping as canonical::Message>::fetch(&buf), Some(ping));
    assert_eq!(<Ping as facade::Message>::fetch(&buf), Some(ping));
    assert_eq!(<Ping as canonical::Message>::fetch(&buf[..3]), None);
}

#[test]
fn facade_and_canonical_contract_bounds_transfer_both_ways() {
    facade_contract_is_canonical::<Ping>();
    canonical_contract_is_facade::<Ping>();
    assert_eq!(contract_request_is_same_type::<Ping>(Ping(7)), Ping(7));

    let buf = message_buf_is_canonical::<Ping>([1, 2, 3, 4]);
    assert_eq!(canonical_message_buf_is_facade::<Ping>(buf), [1, 2, 3, 4]);
    assert_eq!(
        facade_message_is_canonical::<Ping>(&buf),
        Some(Ping(0x0403_0201))
    );
    assert_eq!(canonical_message_is_facade::<Ping>(&buf[..3]), None);
}
