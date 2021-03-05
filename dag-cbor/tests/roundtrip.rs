use std::{io::Cursor, result};

use libipld_cbor::DagCborCodec;
use libipld_core::ipld::Ipld;
use libipld_core::{
    codec::{Codec, Decode},
    raw_value::{IgnoredAny, RawValue, SkipOne},
};

#[test]
fn roundtrip_with_cid() {
    // generated with go-ipfs
    // $ echo foobar > file1
    // $ ipfs add foobar
    // QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL
    // $ echo -n '{ "foo": { "/": "QmRgutAxd8t7oGkSm4wmeuByG6M51wcTso6cubDdQtuEfL" } }' \
    //   | ipfs dag put
    // bafyreibvjvcv745gig4mvqs4hctx4zfkono4rjejm2ta6gtyzkqxfjeily
    // $ ipfs block get bafyreibvjvcv745gig4mvqs4hctx4zfkono4rjejm2ta6gtyzkqxfjeily \
    //   | xxd -ps | paste -s --delimiters=

    let input =
        "a163666f6fd82a582300122031c3d57080d8463a3c63b2923df5a1d40ad7a73eae5a14af584213e5f504ac33";
    let input = hex::decode(input).unwrap();

    let ipld: Ipld = DagCborCodec.decode(&input).unwrap();
    let bytes = DagCborCodec.encode(&ipld).unwrap().to_vec();

    assert_eq!(input, bytes);
}

#[test]
#[should_panic]
fn invalid_cid_prefix() {
    let input =
        "a163666f6fd82a582301122031c3d57080d8463a3c63b2923df5a1d40ad7a73eae5a14af584213e5f504ac33";
    let input = hex::decode(input).unwrap();
    let _: Ipld = DagCborCodec.decode(&input).unwrap();
}

#[test]
#[should_panic]
fn zero_length_cid() {
    let input = "a163666f6fd82a5800";
    let input = hex::decode(input).unwrap();
    let _: Ipld = DagCborCodec.decode(&input).unwrap();
}

// test SkipOne trait for cbor
#[test]
fn skip() {
    // 3x some cbor and then some garbage
    let input = "a163666f6fd82a5800a163666f6fd82a5800ffffff";
    let input = hex::decode(input).unwrap();
    let mut r = Cursor::new(&input);
    DagCborCodec.skip(&mut r).unwrap();
    assert_eq!(r.position(), 9);
    DagCborCodec.skip(&mut r).unwrap();
    assert_eq!(r.position(), 18);
    assert!(DagCborCodec.skip(&mut r).is_err());
}

// test IgnoredAny, which does use skip internally
#[test]
fn ignored_any() {
    // 3x some cbor and then some garbage
    let input = "a163666f6fd82a5800a163666f6fd82a5800ffffff";
    let input = hex::decode(input).unwrap();
    let mut r = Cursor::new(&input);
    let _x: IgnoredAny = Decode::decode(DagCborCodec, &mut r).unwrap();
    assert_eq!(r.position(), 9);
    let _x: IgnoredAny = Decode::decode(DagCborCodec, &mut r).unwrap();
    assert_eq!(r.position(), 18);
    let r: result::Result<IgnoredAny, _> = Decode::decode(DagCborCodec, &mut r);
    assert!(r.is_err());
}

// test RawValue, which does use skip internally
#[test]
fn raw_value() {
    // 3x some cbor and then some garbage
    let input = "a163666f6fd82a5800a163666f6fd82a5800ffffff";
    let input = hex::decode(input).unwrap();
    let mut r = Cursor::new(&input);
    let raw: RawValue<DagCborCodec> = Decode::decode(DagCborCodec, &mut r).unwrap();
    assert_eq!(r.position(), 9);
    assert_eq!(raw.as_ref(), &hex::decode("a163666f6fd82a5800").unwrap());
    let raw: RawValue<DagCborCodec> = Decode::decode(DagCborCodec, &mut r).unwrap();
    assert_eq!(r.position(), 18);
    assert_eq!(raw.as_ref(), &hex::decode("a163666f6fd82a5800").unwrap());
    let r: result::Result<RawValue<DagCborCodec>, _> = Decode::decode(DagCborCodec, &mut r);
    assert!(r.is_err());
}
