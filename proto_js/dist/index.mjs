function vt() {
  let e = 0, t = 0;
  for (let u = 0; u < 28; u += 7) {
    let i = this.buf[this.pos++];
    if (e |= (i & 127) << u, (i & 128) == 0)
      return this.assertBounds(), [e, t];
  }
  let r = this.buf[this.pos++];
  if (e |= (r & 15) << 28, t = (r & 112) >> 4, (r & 128) == 0)
    return this.assertBounds(), [e, t];
  for (let u = 3; u <= 31; u += 7) {
    let i = this.buf[this.pos++];
    if (t |= (i & 127) << u, (i & 128) == 0)
      return this.assertBounds(), [e, t];
  }
  throw new Error("invalid varint");
}
function ie(e, t, r) {
  for (let n = 0; n < 28; n = n + 7) {
    const s = e >>> n, a = !(!(s >>> 7) && t == 0), b = (a ? s | 128 : s) & 255;
    if (r.push(b), !a)
      return;
  }
  const u = e >>> 28 & 15 | (t & 7) << 4, i = t >> 3 != 0;
  if (r.push((i ? u | 128 : u) & 255), !!i) {
    for (let n = 3; n < 31; n = n + 7) {
      const s = t >>> n, a = !!(s >>> 7), b = (a ? s | 128 : s) & 255;
      if (r.push(b), !a)
        return;
    }
    r.push(t >>> 31 & 1);
  }
}
const Z = 4294967296;
function de(e) {
  const t = e[0] === "-";
  t && (e = e.slice(1));
  const r = 1e6;
  let u = 0, i = 0;
  function n(s, a) {
    const b = Number(e.slice(s, a));
    i *= r, u = u * r + b, u >= Z && (i = i + (u / Z | 0), u = u % Z);
  }
  return n(-24, -18), n(-18, -12), n(-12, -6), n(-6), t ? Qe(u, i) : se(u, i);
}
function bt(e, t) {
  let r = se(e, t);
  const u = r.hi & 2147483648;
  u && (r = Qe(r.lo, r.hi));
  const i = Ze(r.lo, r.hi);
  return u ? "-" + i : i;
}
function Ze(e, t) {
  if ({ lo: e, hi: t } = yt(e, t), t <= 2097151)
    return String(Z * t + e);
  const r = e & 16777215, u = (e >>> 24 | t << 8) & 16777215, i = t >> 16 & 65535;
  let n = r + u * 6777216 + i * 6710656, s = u + i * 8147497, a = i * 2;
  const b = 1e7;
  return n >= b && (s += Math.floor(n / b), n %= b), s >= b && (a += Math.floor(s / b), s %= b), a.toString() + ce(s) + ce(n);
}
function yt(e, t) {
  return { lo: e >>> 0, hi: t >>> 0 };
}
function se(e, t) {
  return { lo: e | 0, hi: t | 0 };
}
function Qe(e, t) {
  return t = ~t, e ? e = ~e + 1 : t += 1, se(e, t);
}
const ce = (e) => {
  const t = String(e);
  return "0000000".slice(t.length) + t;
};
function he(e, t) {
  if (e >= 0) {
    for (; e > 127; )
      t.push(e & 127 | 128), e = e >>> 7;
    t.push(e);
  } else {
    for (let r = 0; r < 9; r++)
      t.push(e & 127 | 128), e = e >> 7;
    t.push(1);
  }
}
function mt() {
  let e = this.buf[this.pos++], t = e & 127;
  if ((e & 128) == 0)
    return this.assertBounds(), t;
  if (e = this.buf[this.pos++], t |= (e & 127) << 7, (e & 128) == 0)
    return this.assertBounds(), t;
  if (e = this.buf[this.pos++], t |= (e & 127) << 14, (e & 128) == 0)
    return this.assertBounds(), t;
  if (e = this.buf[this.pos++], t |= (e & 127) << 21, (e & 128) == 0)
    return this.assertBounds(), t;
  e = this.buf[this.pos++], t |= (e & 15) << 28;
  for (let r = 5; (e & 128) !== 0 && r < 10; r++)
    e = this.buf[this.pos++];
  if ((e & 128) != 0)
    throw new Error("invalid varint");
  return this.assertBounds(), t >>> 0;
}
const I = /* @__PURE__ */ xt();
function xt() {
  const e = new DataView(new ArrayBuffer(8));
  if (typeof BigInt == "function" && typeof e.getBigInt64 == "function" && typeof e.getBigUint64 == "function" && typeof e.setBigInt64 == "function" && typeof e.setBigUint64 == "function" && (!!globalThis.Deno || typeof process != "object" || typeof process.env != "object" || process.env.BUF_BIGINT_DISABLE !== "1")) {
    const r = BigInt("-9223372036854775808"), u = BigInt("9223372036854775807"), i = BigInt("0"), n = BigInt("18446744073709551615");
    return {
      zero: BigInt(0),
      supported: !0,
      parse(s) {
        const a = typeof s == "bigint" ? s : BigInt(s);
        if (a > u || a < r)
          throw new Error(`invalid int64: ${s}`);
        return a;
      },
      uParse(s) {
        const a = typeof s == "bigint" ? s : BigInt(s);
        if (a > n || a < i)
          throw new Error(`invalid uint64: ${s}`);
        return a;
      },
      enc(s) {
        return e.setBigInt64(0, this.parse(s), !0), {
          lo: e.getInt32(0, !0),
          hi: e.getInt32(4, !0)
        };
      },
      uEnc(s) {
        return e.setBigInt64(0, this.uParse(s), !0), {
          lo: e.getInt32(0, !0),
          hi: e.getInt32(4, !0)
        };
      },
      dec(s, a) {
        return e.setInt32(0, s, !0), e.setInt32(4, a, !0), e.getBigInt64(0, !0);
      },
      uDec(s, a) {
        return e.setInt32(0, s, !0), e.setInt32(4, a, !0), e.getBigUint64(0, !0);
      }
    };
  }
  return {
    zero: "0",
    supported: !1,
    parse(r) {
      return typeof r != "string" && (r = r.toString()), pe(r), r;
    },
    uParse(r) {
      return typeof r != "string" && (r = r.toString()), ve(r), r;
    },
    enc(r) {
      return typeof r != "string" && (r = r.toString()), pe(r), de(r);
    },
    uEnc(r) {
      return typeof r != "string" && (r = r.toString()), ve(r), de(r);
    },
    dec(r, u) {
      return bt(r, u);
    },
    uDec(r, u) {
      return Ze(r, u);
    }
  };
}
function pe(e) {
  if (!/^-?[0-9]+$/.test(e))
    throw new Error("invalid int64: " + e);
}
function ve(e) {
  if (!/^[0-9]+$/.test(e))
    throw new Error("invalid uint64: " + e);
}
const oe = /* @__PURE__ */ Symbol.for("@bufbuild/protobuf/text-encoding");
function je() {
  if (globalThis[oe] == null) {
    const e = new globalThis.TextEncoder(), t = new globalThis.TextDecoder();
    globalThis[oe] = {
      encodeUtf8(r) {
        return e.encode(r);
      },
      decodeUtf8(r) {
        return t.decode(r);
      },
      checkUtf8(r) {
        try {
          return encodeURIComponent(r), !0;
        } catch {
          return !1;
        }
      }
    };
  }
  return globalThis[oe];
}
var H;
(function(e) {
  e[e.Varint = 0] = "Varint", e[e.Bit64 = 1] = "Bit64", e[e.LengthDelimited = 2] = "LengthDelimited", e[e.StartGroup = 3] = "StartGroup", e[e.EndGroup = 4] = "EndGroup", e[e.Bit32 = 5] = "Bit32";
})(H || (H = {}));
const gt = 34028234663852886e22, wt = -34028234663852886e22, St = 4294967295, Et = 2147483647, kt = -2147483648;
class E {
  constructor(t = je().encodeUtf8) {
    this.encodeUtf8 = t, this.stack = [], this.chunks = [], this.buf = [];
  }
  /**
   * Return all bytes written and reset this writer.
   */
  finish() {
    this.buf.length && (this.chunks.push(new Uint8Array(this.buf)), this.buf = []);
    let t = 0;
    for (let i = 0; i < this.chunks.length; i++)
      t += this.chunks[i].length;
    let r = new Uint8Array(t), u = 0;
    for (let i = 0; i < this.chunks.length; i++)
      r.set(this.chunks[i], u), u += this.chunks[i].length;
    return this.chunks = [], r;
  }
  /**
   * Start a new fork for length-delimited data like a message
   * or a packed repeated field.
   *
   * Must be joined later with `join()`.
   */
  fork() {
    return this.stack.push({ chunks: this.chunks, buf: this.buf }), this.chunks = [], this.buf = [], this;
  }
  /**
   * Join the last fork. Write its length and bytes, then
   * return to the previous state.
   */
  join() {
    let t = this.finish(), r = this.stack.pop();
    if (!r)
      throw new Error("invalid state, fork stack empty");
    return this.chunks = r.chunks, this.buf = r.buf, this.uint32(t.byteLength), this.raw(t);
  }
  /**
   * Writes a tag (field number and wire type).
   *
   * Equivalent to `uint32( (fieldNo << 3 | type) >>> 0 )`.
   *
   * Generated code should compute the tag ahead of time and call `uint32()`.
   */
  tag(t, r) {
    return this.uint32((t << 3 | r) >>> 0);
  }
  /**
   * Write a chunk of raw bytes.
   */
  raw(t) {
    return this.buf.length && (this.chunks.push(new Uint8Array(this.buf)), this.buf = []), this.chunks.push(t), this;
  }
  /**
   * Write a `uint32` value, an unsigned 32 bit varint.
   */
  uint32(t) {
    for (be(t); t > 127; )
      this.buf.push(t & 127 | 128), t = t >>> 7;
    return this.buf.push(t), this;
  }
  /**
   * Write a `int32` value, a signed 32 bit varint.
   */
  int32(t) {
    return ae(t), he(t, this.buf), this;
  }
  /**
   * Write a `bool` value, a variant.
   */
  bool(t) {
    return this.buf.push(t ? 1 : 0), this;
  }
  /**
   * Write a `bytes` value, length-delimited arbitrary data.
   */
  bytes(t) {
    return this.uint32(t.byteLength), this.raw(t);
  }
  /**
   * Write a `string` value, length-delimited data converted to UTF-8 text.
   */
  string(t) {
    let r = this.encodeUtf8(t);
    return this.uint32(r.byteLength), this.raw(r);
  }
  /**
   * Write a `float` value, 32-bit floating point number.
   */
  float(t) {
    Nt(t);
    let r = new Uint8Array(4);
    return new DataView(r.buffer).setFloat32(0, t, !0), this.raw(r);
  }
  /**
   * Write a `double` value, a 64-bit floating point number.
   */
  double(t) {
    let r = new Uint8Array(8);
    return new DataView(r.buffer).setFloat64(0, t, !0), this.raw(r);
  }
  /**
   * Write a `fixed32` value, an unsigned, fixed-length 32-bit integer.
   */
  fixed32(t) {
    be(t);
    let r = new Uint8Array(4);
    return new DataView(r.buffer).setUint32(0, t, !0), this.raw(r);
  }
  /**
   * Write a `sfixed32` value, a signed, fixed-length 32-bit integer.
   */
  sfixed32(t) {
    ae(t);
    let r = new Uint8Array(4);
    return new DataView(r.buffer).setInt32(0, t, !0), this.raw(r);
  }
  /**
   * Write a `sint32` value, a signed, zigzag-encoded 32-bit varint.
   */
  sint32(t) {
    return ae(t), t = (t << 1 ^ t >> 31) >>> 0, he(t, this.buf), this;
  }
  /**
   * Write a `fixed64` value, a signed, fixed-length 64-bit integer.
   */
  sfixed64(t) {
    let r = new Uint8Array(8), u = new DataView(r.buffer), i = I.enc(t);
    return u.setInt32(0, i.lo, !0), u.setInt32(4, i.hi, !0), this.raw(r);
  }
  /**
   * Write a `fixed64` value, an unsigned, fixed-length 64 bit integer.
   */
  fixed64(t) {
    let r = new Uint8Array(8), u = new DataView(r.buffer), i = I.uEnc(t);
    return u.setInt32(0, i.lo, !0), u.setInt32(4, i.hi, !0), this.raw(r);
  }
  /**
   * Write a `int64` value, a signed 64-bit varint.
   */
  int64(t) {
    let r = I.enc(t);
    return ie(r.lo, r.hi, this.buf), this;
  }
  /**
   * Write a `sint64` value, a signed, zig-zag-encoded 64-bit varint.
   */
  sint64(t) {
    const r = I.enc(t), u = r.hi >> 31, i = r.lo << 1 ^ u, n = (r.hi << 1 | r.lo >>> 31) ^ u;
    return ie(i, n, this.buf), this;
  }
  /**
   * Write a `uint64` value, an unsigned 64-bit varint.
   */
  uint64(t) {
    const r = I.uEnc(t);
    return ie(r.lo, r.hi, this.buf), this;
  }
}
class g {
  constructor(t, r = je().decodeUtf8) {
    this.decodeUtf8 = r, this.varint64 = vt, this.uint32 = mt, this.buf = t, this.len = t.length, this.pos = 0, this.view = new DataView(t.buffer, t.byteOffset, t.byteLength);
  }
  /**
   * Reads a tag - field number and wire type.
   */
  tag() {
    let t = this.uint32(), r = t >>> 3, u = t & 7;
    if (r <= 0 || u < 0 || u > 5)
      throw new Error("illegal tag: field no " + r + " wire type " + u);
    return [r, u];
  }
  /**
   * Skip one element and return the skipped data.
   *
   * When skipping StartGroup, provide the tags field number to check for
   * matching field number in the EndGroup tag.
   */
  skip(t, r) {
    let u = this.pos;
    switch (t) {
      case H.Varint:
        for (; this.buf[this.pos++] & 128; )
          ;
        break;
      // @ts-ignore TS7029: Fallthrough case in switch -- ignore instead of expect-error for compiler settings without noFallthroughCasesInSwitch: true
      case H.Bit64:
        this.pos += 4;
      case H.Bit32:
        this.pos += 4;
        break;
      case H.LengthDelimited:
        let i = this.uint32();
        this.pos += i;
        break;
      case H.StartGroup:
        for (; ; ) {
          const [n, s] = this.tag();
          if (s === H.EndGroup) {
            if (r !== void 0 && n !== r)
              throw new Error("invalid end group tag");
            break;
          }
          this.skip(s, n);
        }
        break;
      default:
        throw new Error("cant skip wire type " + t);
    }
    return this.assertBounds(), this.buf.subarray(u, this.pos);
  }
  /**
   * Throws error if position in byte array is out of range.
   */
  assertBounds() {
    if (this.pos > this.len)
      throw new RangeError("premature EOF");
  }
  /**
   * Read a `int32` field, a signed 32 bit varint.
   */
  int32() {
    return this.uint32() | 0;
  }
  /**
   * Read a `sint32` field, a signed, zigzag-encoded 32-bit varint.
   */
  sint32() {
    let t = this.uint32();
    return t >>> 1 ^ -(t & 1);
  }
  /**
   * Read a `int64` field, a signed 64-bit varint.
   */
  int64() {
    return I.dec(...this.varint64());
  }
  /**
   * Read a `uint64` field, an unsigned 64-bit varint.
   */
  uint64() {
    return I.uDec(...this.varint64());
  }
  /**
   * Read a `sint64` field, a signed, zig-zag-encoded 64-bit varint.
   */
  sint64() {
    let [t, r] = this.varint64(), u = -(t & 1);
    return t = (t >>> 1 | (r & 1) << 31) ^ u, r = r >>> 1 ^ u, I.dec(t, r);
  }
  /**
   * Read a `bool` field, a variant.
   */
  bool() {
    let [t, r] = this.varint64();
    return t !== 0 || r !== 0;
  }
  /**
   * Read a `fixed32` field, an unsigned, fixed-length 32-bit integer.
   */
  fixed32() {
    return this.view.getUint32((this.pos += 4) - 4, !0);
  }
  /**
   * Read a `sfixed32` field, a signed, fixed-length 32-bit integer.
   */
  sfixed32() {
    return this.view.getInt32((this.pos += 4) - 4, !0);
  }
  /**
   * Read a `fixed64` field, an unsigned, fixed-length 64 bit integer.
   */
  fixed64() {
    return I.uDec(this.sfixed32(), this.sfixed32());
  }
  /**
   * Read a `fixed64` field, a signed, fixed-length 64-bit integer.
   */
  sfixed64() {
    return I.dec(this.sfixed32(), this.sfixed32());
  }
  /**
   * Read a `float` field, 32-bit floating point number.
   */
  float() {
    return this.view.getFloat32((this.pos += 4) - 4, !0);
  }
  /**
   * Read a `double` field, a 64-bit floating point number.
   */
  double() {
    return this.view.getFloat64((this.pos += 8) - 8, !0);
  }
  /**
   * Read a `bytes` field, length-delimited arbitrary data.
   */
  bytes() {
    let t = this.uint32(), r = this.pos;
    return this.pos += t, this.assertBounds(), this.buf.subarray(r, r + t);
  }
  /**
   * Read a `string` field, length-delimited data converted to UTF-8 text.
   */
  string() {
    return this.decodeUtf8(this.bytes());
  }
}
function ae(e) {
  if (typeof e == "string")
    e = Number(e);
  else if (typeof e != "number")
    throw new Error("invalid int32: " + typeof e);
  if (!Number.isInteger(e) || e > Et || e < kt)
    throw new Error("invalid int32: " + e);
}
function be(e) {
  if (typeof e == "string")
    e = Number(e);
  else if (typeof e != "number")
    throw new Error("invalid uint32: " + typeof e);
  if (!Number.isInteger(e) || e > St || e < 0)
    throw new Error("invalid uint32: " + e);
}
function Nt(e) {
  if (typeof e == "string") {
    const t = e;
    if (e = Number(e), Number.isNaN(e) && t !== "NaN")
      throw new Error("invalid float32: " + t);
  } else if (typeof e != "number")
    throw new Error("invalid float32: " + typeof e);
  if (Number.isFinite(e) && (e > gt || e < wt))
    throw new Error("invalid float32: " + e);
}
function ye() {
  return { t: 0, x: 0, y: 0, heading: 0, vl: 0, vr: 0, omega: 0, al: 0, ar: 0, alpha: 0, fl: 0, fr: 0 };
}
const U = {
  encode(e, t = new E()) {
    return e.t !== 0 && t.uint32(9).double(e.t), e.x !== 0 && t.uint32(17).double(e.x), e.y !== 0 && t.uint32(25).double(e.y), e.heading !== 0 && t.uint32(33).double(e.heading), e.vl !== 0 && t.uint32(41).double(e.vl), e.vr !== 0 && t.uint32(49).double(e.vr), e.omega !== 0 && t.uint32(57).double(e.omega), e.al !== 0 && t.uint32(65).double(e.al), e.ar !== 0 && t.uint32(73).double(e.ar), e.alpha !== 0 && t.uint32(81).double(e.alpha), e.fl !== 0 && t.uint32(89).double(e.fl), e.fr !== 0 && t.uint32(97).double(e.fr), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = ye();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 9)
            break;
          i.t = r.double();
          continue;
        }
        case 2: {
          if (n !== 17)
            break;
          i.x = r.double();
          continue;
        }
        case 3: {
          if (n !== 25)
            break;
          i.y = r.double();
          continue;
        }
        case 4: {
          if (n !== 33)
            break;
          i.heading = r.double();
          continue;
        }
        case 5: {
          if (n !== 41)
            break;
          i.vl = r.double();
          continue;
        }
        case 6: {
          if (n !== 49)
            break;
          i.vr = r.double();
          continue;
        }
        case 7: {
          if (n !== 57)
            break;
          i.omega = r.double();
          continue;
        }
        case 8: {
          if (n !== 65)
            break;
          i.al = r.double();
          continue;
        }
        case 9: {
          if (n !== 73)
            break;
          i.ar = r.double();
          continue;
        }
        case 10: {
          if (n !== 81)
            break;
          i.alpha = r.double();
          continue;
        }
        case 11: {
          if (n !== 89)
            break;
          i.fl = r.double();
          continue;
        }
        case 12: {
          if (n !== 97)
            break;
          i.fr = r.double();
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      t: A(e.t) ? globalThis.Number(e.t) : 0,
      x: A(e.x) ? globalThis.Number(e.x) : 0,
      y: A(e.y) ? globalThis.Number(e.y) : 0,
      heading: A(e.heading) ? globalThis.Number(e.heading) : 0,
      vl: A(e.vl) ? globalThis.Number(e.vl) : 0,
      vr: A(e.vr) ? globalThis.Number(e.vr) : 0,
      omega: A(e.omega) ? globalThis.Number(e.omega) : 0,
      al: A(e.al) ? globalThis.Number(e.al) : 0,
      ar: A(e.ar) ? globalThis.Number(e.ar) : 0,
      alpha: A(e.alpha) ? globalThis.Number(e.alpha) : 0,
      fl: A(e.fl) ? globalThis.Number(e.fl) : 0,
      fr: A(e.fr) ? globalThis.Number(e.fr) : 0
    };
  },
  toJSON(e) {
    const t = {};
    return e.t !== 0 && (t.t = e.t), e.x !== 0 && (t.x = e.x), e.y !== 0 && (t.y = e.y), e.heading !== 0 && (t.heading = e.heading), e.vl !== 0 && (t.vl = e.vl), e.vr !== 0 && (t.vr = e.vr), e.omega !== 0 && (t.omega = e.omega), e.al !== 0 && (t.al = e.al), e.ar !== 0 && (t.ar = e.ar), e.alpha !== 0 && (t.alpha = e.alpha), e.fl !== 0 && (t.fl = e.fl), e.fr !== 0 && (t.fr = e.fr), t;
  },
  create(e) {
    return U.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = ye();
    return t.t = e.t ?? 0, t.x = e.x ?? 0, t.y = e.y ?? 0, t.heading = e.heading ?? 0, t.vl = e.vl ?? 0, t.vr = e.vr ?? 0, t.omega = e.omega ?? 0, t.al = e.al ?? 0, t.ar = e.ar ?? 0, t.alpha = e.alpha ?? 0, t.fl = e.fl ?? 0, t.fr = e.fr ?? 0, t;
  }
};
function A(e) {
  return e != null;
}
var et = /* @__PURE__ */ ((e) => (e[e.DRIVETYPE_SWERVE = 0] = "DRIVETYPE_SWERVE", e[e.DRIVETYPE_DIFFERENTIAL = 1] = "DRIVETYPE_DIFFERENTIAL", e[e.DRIVETYPE_MECANUM = 2] = "DRIVETYPE_MECANUM", e[e.UNRECOGNIZED = -1] = "UNRECOGNIZED", e))(et || {});
function Tt(e) {
  switch (e) {
    case 0:
    case "DRIVETYPE_SWERVE":
      return 0;
    case 1:
    case "DRIVETYPE_DIFFERENTIAL":
      return 1;
    case 2:
    case "DRIVETYPE_MECANUM":
      return 2;
    default:
      return -1;
  }
}
function Ot(e) {
  switch (e) {
    case 0:
      return "DRIVETYPE_SWERVE";
    case 1:
      return "DRIVETYPE_DIFFERENTIAL";
    case 2:
      return "DRIVETYPE_MECANUM";
    default:
      return "UNRECOGNIZED";
  }
}
function me() {
  return { max: 0 };
}
const C = {
  encode(e, t = new E()) {
    return e.max !== 0 && t.uint32(9).double(e.max), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = me();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 9)
            break;
          i.max = r.double();
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return { max: rt(e.max) ? globalThis.Number(e.max) : 0 };
  },
  toJSON(e) {
    const t = {};
    return e.max !== 0 && (t.max = e.max), t;
  },
  create(e) {
    return C.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = me();
    return t.max = e.max ?? 0, t;
  }
};
function xe() {
  return { test: "" };
}
const tt = {
  encode(e, t = new E()) {
    return e.test !== "" && t.uint32(10).string(e.test), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = xe();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.test = r.string();
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return { test: rt(e.test) ? globalThis.String(e.test) : "" };
  },
  toJSON(e) {
    const t = {};
    return e.test !== "" && (t.test = e.test), t;
  },
  create(e) {
    return tt.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = xe();
    return t.test = e.test ?? "", t;
  }
};
function rt(e) {
  return e != null;
}
function ge() {
  return { value: 0, expr: "" };
}
const S = {
  encode(e, t = new E()) {
    return e.value !== 0 && t.uint32(9).double(e.value), e.expr !== "" && t.uint32(18).string(e.expr), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = ge();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 9)
            break;
          i.value = r.double();
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.expr = r.string();
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      value: we(e.value) ? globalThis.Number(e.value) : 0,
      expr: we(e.expr) ? globalThis.String(e.expr) : ""
    };
  },
  toJSON(e) {
    const t = {};
    return e.value !== 0 && (t.value = e.value), e.expr !== "" && (t.expr = e.expr), t;
  },
  create(e) {
    return S.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = ge();
    return t.value = e.value ?? 0, t.expr = e.expr ?? "", t;
  }
};
function we(e) {
  return e != null;
}
function Se() {
  return { max: void 0 };
}
const J = {
  encode(e, t = new E()) {
    return e.max !== void 0 && S.encode(e.max, t.uint32(10).fork()).join(), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Se();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.max = S.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return { max: it(e.max) ? S.fromJSON(e.max) : void 0 };
  },
  toJSON(e) {
    const t = {};
    return e.max !== void 0 && (t.max = S.toJSON(e.max)), t;
  },
  create(e) {
    return J.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Se();
    return t.max = e.max !== void 0 && e.max !== null ? S.fromPartial(e.max) : void 0, t;
  }
};
function Ee() {
  return { test: "" };
}
const nt = {
  encode(e, t = new E()) {
    return e.test !== "" && t.uint32(10).string(e.test), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Ee();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.test = r.string();
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return { test: it(e.test) ? globalThis.String(e.test) : "" };
  },
  toJSON(e) {
    const t = {};
    return e.test !== "" && (t.test = e.test), t;
  },
  create(e) {
    return nt.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Ee();
    return t.test = e.test ?? "", t;
  }
};
function it(e) {
  return e != null;
}
const _t = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DoubleMaxVelocity: C,
  ExprMaxVelocity: J,
  TestDouble: tt,
  TestExpr: nt
}, Symbol.toStringTag, { value: "Module" }));
function ke() {
  return { max: 0 };
}
const R = {
  encode(e, t = new E()) {
    return e.max !== 0 && t.uint32(9).double(e.max), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = ke();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 9)
            break;
          i.max = r.double();
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return { max: At(e.max) ? globalThis.Number(e.max) : 0 };
  },
  toJSON(e) {
    const t = {};
    return e.max !== 0 && (t.max = e.max), t;
  },
  create(e) {
    return R.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = ke();
    return t.max = e.max ?? 0, t;
  }
};
function At(e) {
  return e != null;
}
function Ne() {
  return { max: void 0 };
}
const B = {
  encode(e, t = new E()) {
    return e.max !== void 0 && S.encode(e.max, t.uint32(10).fork()).join(), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Ne();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.max = S.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return { max: Mt(e.max) ? S.fromJSON(e.max) : void 0 };
  },
  toJSON(e) {
    const t = {};
    return e.max !== void 0 && (t.max = S.toJSON(e.max)), t;
  },
  create(e) {
    return B.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Ne();
    return t.max = e.max !== void 0 && e.max !== null ? S.fromPartial(e.max) : void 0, t;
  }
};
function Mt(e) {
  return e != null;
}
const Pt = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DoubleMaxAcceleration: R,
  ExprMaxAcceleration: B
}, Symbol.toStringTag, { value: "Module" }));
function Te() {
  return {};
}
const $ = {
  encode(e, t = new E()) {
    return t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Te();
    for (; r.pos < u; ) {
      const n = r.uint32();
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {};
  },
  toJSON(e) {
    return {};
  },
  create(e) {
    return $.fromPartial(e ?? {});
  },
  fromPartial(e) {
    return Te();
  }
};
function Oe() {
  return {};
}
const z = {
  encode(e, t = new E()) {
    return t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Oe();
    for (; r.pos < u; ) {
      const n = r.uint32();
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {};
  },
  toJSON(e) {
    return {};
  },
  create(e) {
    return z.fromPartial(e ?? {});
  },
  fromPartial(e) {
    return Oe();
  }
};
function _e() {
  return { idx: 0 };
}
const X = {
  encode(e, t = new E()) {
    return e.idx !== 0 && t.uint32(8).uint64(e.idx), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = _e();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 8)
            break;
          i.idx = It(r.uint64());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return { idx: Q(e.idx) ? globalThis.Number(e.idx) : 0 };
  },
  toJSON(e) {
    const t = {};
    return e.idx !== 0 && (t.idx = Math.round(e.idx)), t;
  },
  create(e) {
    return X.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = _e();
    return t.idx = e.idx ?? 0, t;
  }
};
function Ae() {
  return { id: void 0 };
}
const N = {
  encode(e, t = new E()) {
    switch (e.id?.$case) {
      case "first":
        $.encode(e.id.first, t.uint32(10).fork()).join();
        break;
      case "last":
        z.encode(e.id.last, t.uint32(18).fork()).join();
        break;
      case "idx":
        X.encode(e.id.idx, t.uint32(26).fork()).join();
        break;
    }
    return t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Ae();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.id = { $case: "first", first: $.decode(r, r.uint32()) };
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.id = { $case: "last", last: z.decode(r, r.uint32()) };
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.id = { $case: "idx", idx: X.decode(r, r.uint32()) };
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      id: Q(e.first) ? { $case: "first", first: $.fromJSON(e.first) } : Q(e.last) ? { $case: "last", last: z.fromJSON(e.last) } : Q(e.idx) ? { $case: "idx", idx: X.fromJSON(e.idx) } : void 0
    };
  },
  toJSON(e) {
    const t = {};
    return e.id?.$case === "first" ? t.first = $.toJSON(e.id.first) : e.id?.$case === "last" ? t.last = z.toJSON(e.id.last) : e.id?.$case === "idx" && (t.idx = X.toJSON(e.id.idx)), t;
  },
  create(e) {
    return N.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Ae();
    switch (e.id?.$case) {
      case "first": {
        e.id?.first !== void 0 && e.id?.first !== null && (t.id = { $case: "first", first: $.fromPartial(e.id.first) });
        break;
      }
      case "last": {
        e.id?.last !== void 0 && e.id?.last !== null && (t.id = { $case: "last", last: z.fromPartial(e.id.last) });
        break;
      }
      case "idx": {
        e.id?.idx !== void 0 && e.id?.idx !== null && (t.id = { $case: "idx", idx: X.fromPartial(e.id.idx) });
        break;
      }
    }
    return t;
  }
};
function It(e) {
  const t = globalThis.Number(e.toString());
  if (t > globalThis.Number.MAX_SAFE_INTEGER)
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  if (t < globalThis.Number.MIN_SAFE_INTEGER)
    throw new globalThis.Error("Value is smaller than Number.MIN_SAFE_INTEGER");
  return t;
}
function Q(e) {
  return e != null;
}
function Me() {
  return { enabled: !1, from: void 0, to: void 0, data: void 0 };
}
const L = {
  encode(e, t = new E()) {
    switch (e.enabled !== !1 && t.uint32(8).bool(e.enabled), e.from !== void 0 && N.encode(e.from, t.uint32(18).fork()).join(), e.to !== void 0 && N.encode(e.to, t.uint32(26).fork()).join(), e.data?.$case) {
      case "maxVelocity":
        C.encode(e.data.maxVelocity, t.uint32(34).fork()).join();
        break;
      case "maxAcceleration":
        R.encode(e.data.maxAcceleration, t.uint32(42).fork()).join();
        break;
    }
    return t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Me();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 8)
            break;
          i.enabled = r.bool();
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.from = N.decode(r, r.uint32());
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.to = N.decode(r, r.uint32());
          continue;
        }
        case 4: {
          if (n !== 34)
            break;
          i.data = { $case: "maxVelocity", maxVelocity: C.decode(r, r.uint32()) };
          continue;
        }
        case 5: {
          if (n !== 42)
            break;
          i.data = {
            $case: "maxAcceleration",
            maxAcceleration: R.decode(r, r.uint32())
          };
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      enabled: V(e.enabled) ? globalThis.Boolean(e.enabled) : !1,
      from: V(e.from) ? N.fromJSON(e.from) : void 0,
      to: V(e.to) ? N.fromJSON(e.to) : void 0,
      data: V(e.maxVelocity) ? { $case: "maxVelocity", maxVelocity: C.fromJSON(e.maxVelocity) } : V(e.max_velocity) ? { $case: "maxVelocity", maxVelocity: C.fromJSON(e.max_velocity) } : V(e.maxAcceleration) ? { $case: "maxAcceleration", maxAcceleration: R.fromJSON(e.maxAcceleration) } : V(e.max_acceleration) ? { $case: "maxAcceleration", maxAcceleration: R.fromJSON(e.max_acceleration) } : void 0
    };
  },
  toJSON(e) {
    const t = {};
    return e.enabled !== !1 && (t.enabled = e.enabled), e.from !== void 0 && (t.from = N.toJSON(e.from)), e.to !== void 0 && (t.to = N.toJSON(e.to)), e.data?.$case === "maxVelocity" ? t.maxVelocity = C.toJSON(e.data.maxVelocity) : e.data?.$case === "maxAcceleration" && (t.maxAcceleration = R.toJSON(e.data.maxAcceleration)), t;
  },
  create(e) {
    return L.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Me();
    switch (t.enabled = e.enabled ?? !1, t.from = e.from !== void 0 && e.from !== null ? N.fromPartial(e.from) : void 0, t.to = e.to !== void 0 && e.to !== null ? N.fromPartial(e.to) : void 0, e.data?.$case) {
      case "maxVelocity": {
        e.data?.maxVelocity !== void 0 && e.data?.maxVelocity !== null && (t.data = { $case: "maxVelocity", maxVelocity: C.fromPartial(e.data.maxVelocity) });
        break;
      }
      case "maxAcceleration": {
        e.data?.maxAcceleration !== void 0 && e.data?.maxAcceleration !== null && (t.data = {
          $case: "maxAcceleration",
          maxAcceleration: R.fromPartial(e.data.maxAcceleration)
        });
        break;
      }
    }
    return t;
  }
};
function V(e) {
  return e != null;
}
function Pe() {
  return { enabled: !1, from: void 0, to: void 0, data: void 0 };
}
const G = {
  encode(e, t = new E()) {
    switch (e.enabled !== !1 && t.uint32(8).bool(e.enabled), e.from !== void 0 && N.encode(e.from, t.uint32(18).fork()).join(), e.to !== void 0 && N.encode(e.to, t.uint32(26).fork()).join(), e.data?.$case) {
      case "maxVelocity":
        J.encode(e.data.maxVelocity, t.uint32(34).fork()).join();
        break;
      case "maxAcceleration":
        B.encode(e.data.maxAcceleration, t.uint32(42).fork()).join();
        break;
    }
    return t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Pe();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 8)
            break;
          i.enabled = r.bool();
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.from = N.decode(r, r.uint32());
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.to = N.decode(r, r.uint32());
          continue;
        }
        case 4: {
          if (n !== 34)
            break;
          i.data = { $case: "maxVelocity", maxVelocity: J.decode(r, r.uint32()) };
          continue;
        }
        case 5: {
          if (n !== 42)
            break;
          i.data = {
            $case: "maxAcceleration",
            maxAcceleration: B.decode(r, r.uint32())
          };
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      enabled: F(e.enabled) ? globalThis.Boolean(e.enabled) : !1,
      from: F(e.from) ? N.fromJSON(e.from) : void 0,
      to: F(e.to) ? N.fromJSON(e.to) : void 0,
      data: F(e.maxVelocity) ? { $case: "maxVelocity", maxVelocity: J.fromJSON(e.maxVelocity) } : F(e.max_velocity) ? { $case: "maxVelocity", maxVelocity: J.fromJSON(e.max_velocity) } : F(e.maxAcceleration) ? { $case: "maxAcceleration", maxAcceleration: B.fromJSON(e.maxAcceleration) } : F(e.max_acceleration) ? { $case: "maxAcceleration", maxAcceleration: B.fromJSON(e.max_acceleration) } : void 0
    };
  },
  toJSON(e) {
    const t = {};
    return e.enabled !== !1 && (t.enabled = e.enabled), e.from !== void 0 && (t.from = N.toJSON(e.from)), e.to !== void 0 && (t.to = N.toJSON(e.to)), e.data?.$case === "maxVelocity" ? t.maxVelocity = J.toJSON(e.data.maxVelocity) : e.data?.$case === "maxAcceleration" && (t.maxAcceleration = B.toJSON(e.data.maxAcceleration)), t;
  },
  create(e) {
    return G.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Pe();
    switch (t.enabled = e.enabled ?? !1, t.from = e.from !== void 0 && e.from !== null ? N.fromPartial(e.from) : void 0, t.to = e.to !== void 0 && e.to !== null ? N.fromPartial(e.to) : void 0, e.data?.$case) {
      case "maxVelocity": {
        e.data?.maxVelocity !== void 0 && e.data?.maxVelocity !== null && (t.data = { $case: "maxVelocity", maxVelocity: J.fromPartial(e.data.maxVelocity) });
        break;
      }
      case "maxAcceleration": {
        e.data?.maxAcceleration !== void 0 && e.data?.maxAcceleration !== null && (t.data = {
          $case: "maxAcceleration",
          maxAcceleration: B.fromPartial(e.data.maxAcceleration)
        });
        break;
      }
    }
    return t;
  }
};
function F(e) {
  return e != null;
}
const Ht = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DoubleConstraint: L,
  ExprConstraint: G,
  max_acceleration: Pt,
  maxvelocity: _t
}, Symbol.toStringTag, { value: "Module" }));
function Ie() {
  return {
    x: 0,
    y: 0,
    heading: 0,
    intervals: 0,
    split: !1,
    fixTranslation: !1,
    fixHeading: !1,
    overrideIntervals: !1
  };
}
const q = {
  encode(e, t = new E()) {
    return e.x !== 0 && t.uint32(9).double(e.x), e.y !== 0 && t.uint32(17).double(e.y), e.heading !== 0 && t.uint32(25).double(e.heading), e.intervals !== 0 && t.uint32(32).uint64(e.intervals), e.split !== !1 && t.uint32(40).bool(e.split), e.fixTranslation !== !1 && t.uint32(48).bool(e.fixTranslation), e.fixHeading !== !1 && t.uint32(56).bool(e.fixHeading), e.overrideIntervals !== !1 && t.uint32(64).bool(e.overrideIntervals), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Ie();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 9)
            break;
          i.x = r.double();
          continue;
        }
        case 2: {
          if (n !== 17)
            break;
          i.y = r.double();
          continue;
        }
        case 3: {
          if (n !== 25)
            break;
          i.heading = r.double();
          continue;
        }
        case 4: {
          if (n !== 32)
            break;
          i.intervals = Ct(r.uint64());
          continue;
        }
        case 5: {
          if (n !== 40)
            break;
          i.split = r.bool();
          continue;
        }
        case 6: {
          if (n !== 48)
            break;
          i.fixTranslation = r.bool();
          continue;
        }
        case 7: {
          if (n !== 56)
            break;
          i.fixHeading = r.bool();
          continue;
        }
        case 8: {
          if (n !== 64)
            break;
          i.overrideIntervals = r.bool();
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      x: M(e.x) ? globalThis.Number(e.x) : 0,
      y: M(e.y) ? globalThis.Number(e.y) : 0,
      heading: M(e.heading) ? globalThis.Number(e.heading) : 0,
      intervals: M(e.intervals) ? globalThis.Number(e.intervals) : 0,
      split: M(e.split) ? globalThis.Boolean(e.split) : !1,
      fixTranslation: M(e.fixTranslation) ? globalThis.Boolean(e.fixTranslation) : M(e.fix_translation) ? globalThis.Boolean(e.fix_translation) : !1,
      fixHeading: M(e.fixHeading) ? globalThis.Boolean(e.fixHeading) : M(e.fix_heading) ? globalThis.Boolean(e.fix_heading) : !1,
      overrideIntervals: M(e.overrideIntervals) ? globalThis.Boolean(e.overrideIntervals) : M(e.override_intervals) ? globalThis.Boolean(e.override_intervals) : !1
    };
  },
  toJSON(e) {
    const t = {};
    return e.x !== 0 && (t.x = e.x), e.y !== 0 && (t.y = e.y), e.heading !== 0 && (t.heading = e.heading), e.intervals !== 0 && (t.intervals = Math.round(e.intervals)), e.split !== !1 && (t.split = e.split), e.fixTranslation !== !1 && (t.fixTranslation = e.fixTranslation), e.fixHeading !== !1 && (t.fixHeading = e.fixHeading), e.overrideIntervals !== !1 && (t.overrideIntervals = e.overrideIntervals), t;
  },
  create(e) {
    return q.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Ie();
    return t.x = e.x ?? 0, t.y = e.y ?? 0, t.heading = e.heading ?? 0, t.intervals = e.intervals ?? 0, t.split = e.split ?? !1, t.fixTranslation = e.fixTranslation ?? !1, t.fixHeading = e.fixHeading ?? !1, t.overrideIntervals = e.overrideIntervals ?? !1, t;
  }
};
function Ct(e) {
  const t = globalThis.Number(e.toString());
  if (t > globalThis.Number.MAX_SAFE_INTEGER)
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  if (t < globalThis.Number.MIN_SAFE_INTEGER)
    throw new globalThis.Error("Value is smaller than Number.MIN_SAFE_INTEGER");
  return t;
}
function M(e) {
  return e != null;
}
function He() {
  return {
    x: void 0,
    y: void 0,
    heading: void 0,
    intervals: 0,
    split: !1,
    fixTranslation: !1,
    fixHeading: !1,
    overrideIntervals: !1
  };
}
const W = {
  encode(e, t = new E()) {
    return e.x !== void 0 && S.encode(e.x, t.uint32(10).fork()).join(), e.y !== void 0 && S.encode(e.y, t.uint32(18).fork()).join(), e.heading !== void 0 && S.encode(e.heading, t.uint32(26).fork()).join(), e.intervals !== 0 && t.uint32(32).uint64(e.intervals), e.split !== !1 && t.uint32(40).bool(e.split), e.fixTranslation !== !1 && t.uint32(48).bool(e.fixTranslation), e.fixHeading !== !1 && t.uint32(56).bool(e.fixHeading), e.overrideIntervals !== !1 && t.uint32(64).bool(e.overrideIntervals), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = He();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.x = S.decode(r, r.uint32());
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.y = S.decode(r, r.uint32());
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.heading = S.decode(r, r.uint32());
          continue;
        }
        case 4: {
          if (n !== 32)
            break;
          i.intervals = Jt(r.uint64());
          continue;
        }
        case 5: {
          if (n !== 40)
            break;
          i.split = r.bool();
          continue;
        }
        case 6: {
          if (n !== 48)
            break;
          i.fixTranslation = r.bool();
          continue;
        }
        case 7: {
          if (n !== 56)
            break;
          i.fixHeading = r.bool();
          continue;
        }
        case 8: {
          if (n !== 64)
            break;
          i.overrideIntervals = r.bool();
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      x: P(e.x) ? S.fromJSON(e.x) : void 0,
      y: P(e.y) ? S.fromJSON(e.y) : void 0,
      heading: P(e.heading) ? S.fromJSON(e.heading) : void 0,
      intervals: P(e.intervals) ? globalThis.Number(e.intervals) : 0,
      split: P(e.split) ? globalThis.Boolean(e.split) : !1,
      fixTranslation: P(e.fixTranslation) ? globalThis.Boolean(e.fixTranslation) : P(e.fix_translation) ? globalThis.Boolean(e.fix_translation) : !1,
      fixHeading: P(e.fixHeading) ? globalThis.Boolean(e.fixHeading) : P(e.fix_heading) ? globalThis.Boolean(e.fix_heading) : !1,
      overrideIntervals: P(e.overrideIntervals) ? globalThis.Boolean(e.overrideIntervals) : P(e.override_intervals) ? globalThis.Boolean(e.override_intervals) : !1
    };
  },
  toJSON(e) {
    const t = {};
    return e.x !== void 0 && (t.x = S.toJSON(e.x)), e.y !== void 0 && (t.y = S.toJSON(e.y)), e.heading !== void 0 && (t.heading = S.toJSON(e.heading)), e.intervals !== 0 && (t.intervals = Math.round(e.intervals)), e.split !== !1 && (t.split = e.split), e.fixTranslation !== !1 && (t.fixTranslation = e.fixTranslation), e.fixHeading !== !1 && (t.fixHeading = e.fixHeading), e.overrideIntervals !== !1 && (t.overrideIntervals = e.overrideIntervals), t;
  },
  create(e) {
    return W.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = He();
    return t.x = e.x !== void 0 && e.x !== null ? S.fromPartial(e.x) : void 0, t.y = e.y !== void 0 && e.y !== null ? S.fromPartial(e.y) : void 0, t.heading = e.heading !== void 0 && e.heading !== null ? S.fromPartial(e.heading) : void 0, t.intervals = e.intervals ?? 0, t.split = e.split ?? !1, t.fixTranslation = e.fixTranslation ?? !1, t.fixHeading = e.fixHeading ?? !1, t.overrideIntervals = e.overrideIntervals ?? !1, t;
  }
};
function Jt(e) {
  const t = globalThis.Number(e.toString());
  if (t > globalThis.Number.MAX_SAFE_INTEGER)
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  if (t < globalThis.Number.MIN_SAFE_INTEGER)
    throw new globalThis.Error("Value is smaller than Number.MIN_SAFE_INTEGER");
  return t;
}
function P(e) {
  return e != null;
}
const Rt = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DoubleWaypoint: q,
  ExprWaypoint: W
}, Symbol.toStringTag, { value: "Module" }));
function Ce() {
  return { targetDt: 0, waypoints: [], constraints: [] };
}
const ot = {
  encode(e, t = new E()) {
    e.targetDt !== 0 && t.uint32(9).double(e.targetDt);
    for (const r of e.waypoints)
      q.encode(r, t.uint32(18).fork()).join();
    for (const r of e.constraints)
      L.encode(r, t.uint32(26).fork()).join();
    return t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Ce();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 9)
            break;
          i.targetDt = r.double();
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.waypoints.push(q.decode(r, r.uint32()));
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.constraints.push(L.decode(r, r.uint32()));
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      targetDt: Je(e.targetDt) ? globalThis.Number(e.targetDt) : Je(e.target_dt) ? globalThis.Number(e.target_dt) : 0,
      waypoints: globalThis.Array.isArray(e?.waypoints) ? e.waypoints.map((t) => q.fromJSON(t)) : [],
      constraints: globalThis.Array.isArray(e?.constraints) ? e.constraints.map((t) => L.fromJSON(t)) : []
    };
  },
  toJSON(e) {
    const t = {};
    return e.targetDt !== 0 && (t.targetDt = e.targetDt), e.waypoints?.length && (t.waypoints = e.waypoints.map((r) => q.toJSON(r))), e.constraints?.length && (t.constraints = e.constraints.map((r) => L.toJSON(r))), t;
  },
  create(e) {
    return ot.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Ce();
    return t.targetDt = e.targetDt ?? 0, t.waypoints = e.waypoints?.map((r) => q.fromPartial(r)) || [], t.constraints = e.constraints?.map((r) => L.fromPartial(r)) || [], t;
  }
};
function Je(e) {
  return e != null;
}
function Re() {
  return { targetDt: void 0, waypoints: [], constraints: [] };
}
const at = {
  encode(e, t = new E()) {
    e.targetDt !== void 0 && S.encode(e.targetDt, t.uint32(10).fork()).join();
    for (const r of e.waypoints)
      W.encode(r, t.uint32(18).fork()).join();
    for (const r of e.constraints)
      G.encode(r, t.uint32(26).fork()).join();
    return t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Re();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.targetDt = S.decode(r, r.uint32());
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.waypoints.push(W.decode(r, r.uint32()));
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.constraints.push(G.decode(r, r.uint32()));
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      targetDt: Be(e.targetDt) ? S.fromJSON(e.targetDt) : Be(e.target_dt) ? S.fromJSON(e.target_dt) : void 0,
      waypoints: globalThis.Array.isArray(e?.waypoints) ? e.waypoints.map((t) => W.fromJSON(t)) : [],
      constraints: globalThis.Array.isArray(e?.constraints) ? e.constraints.map((t) => G.fromJSON(t)) : []
    };
  },
  toJSON(e) {
    const t = {};
    return e.targetDt !== void 0 && (t.targetDt = S.toJSON(e.targetDt)), e.waypoints?.length && (t.waypoints = e.waypoints.map((r) => W.toJSON(r))), e.constraints?.length && (t.constraints = e.constraints.map((r) => G.toJSON(r))), t;
  },
  create(e) {
    return at.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Re();
    return t.targetDt = e.targetDt !== void 0 && e.targetDt !== null ? S.fromPartial(e.targetDt) : void 0, t.waypoints = e.waypoints?.map((r) => W.fromPartial(r)) || [], t.constraints = e.constraints?.map((r) => G.fromPartial(r)) || [], t;
  }
};
function Be(e) {
  return e != null;
}
const Bt = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DoubleParameters: ot,
  Expr: S,
  ExprParameters: at,
  WaypointID: N,
  WaypointIDFirst: $,
  WaypointIDLast: z,
  WaypointIDX: X,
  constraint: Ht,
  waypoint: Rt
}, Symbol.toStringTag, { value: "Module" }));
function De() {
  return { x: 0, y: 0 };
}
const k = {
  encode(e, t = new E()) {
    return e.x !== 0 && t.uint32(9).double(e.x), e.y !== 0 && t.uint32(17).double(e.y), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = De();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 9)
            break;
          i.x = r.double();
          continue;
        }
        case 2: {
          if (n !== 17)
            break;
          i.y = r.double();
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      x: O(e.x) ? globalThis.Number(e.x) : 0,
      y: O(e.y) ? globalThis.Number(e.y) : 0
    };
  },
  toJSON(e) {
    const t = {};
    return e.x !== 0 && (t.x = e.x), e.y !== 0 && (t.y = e.y), t;
  },
  create(e) {
    return k.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = De();
    return t.x = e.x ?? 0, t.y = e.y ?? 0, t;
  }
};
function Ve() {
  return {
    t: 0,
    x: 0,
    y: 0,
    heading: 0,
    vx: 0,
    vy: 0,
    omega: 0,
    ax: 0,
    ay: 0,
    alpha: 0,
    fl: void 0,
    fr: void 0,
    bl: void 0,
    br: void 0
  };
}
const T = {
  encode(e, t = new E()) {
    return e.t !== 0 && t.uint32(9).double(e.t), e.x !== 0 && t.uint32(17).double(e.x), e.y !== 0 && t.uint32(25).double(e.y), e.heading !== 0 && t.uint32(33).double(e.heading), e.vx !== 0 && t.uint32(41).double(e.vx), e.vy !== 0 && t.uint32(49).double(e.vy), e.omega !== 0 && t.uint32(57).double(e.omega), e.ax !== 0 && t.uint32(65).double(e.ax), e.ay !== 0 && t.uint32(73).double(e.ay), e.alpha !== 0 && t.uint32(81).double(e.alpha), e.fl !== void 0 && k.encode(e.fl, t.uint32(90).fork()).join(), e.fr !== void 0 && k.encode(e.fr, t.uint32(98).fork()).join(), e.bl !== void 0 && k.encode(e.bl, t.uint32(106).fork()).join(), e.br !== void 0 && k.encode(e.br, t.uint32(114).fork()).join(), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Ve();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 9)
            break;
          i.t = r.double();
          continue;
        }
        case 2: {
          if (n !== 17)
            break;
          i.x = r.double();
          continue;
        }
        case 3: {
          if (n !== 25)
            break;
          i.y = r.double();
          continue;
        }
        case 4: {
          if (n !== 33)
            break;
          i.heading = r.double();
          continue;
        }
        case 5: {
          if (n !== 41)
            break;
          i.vx = r.double();
          continue;
        }
        case 6: {
          if (n !== 49)
            break;
          i.vy = r.double();
          continue;
        }
        case 7: {
          if (n !== 57)
            break;
          i.omega = r.double();
          continue;
        }
        case 8: {
          if (n !== 65)
            break;
          i.ax = r.double();
          continue;
        }
        case 9: {
          if (n !== 73)
            break;
          i.ay = r.double();
          continue;
        }
        case 10: {
          if (n !== 81)
            break;
          i.alpha = r.double();
          continue;
        }
        case 11: {
          if (n !== 90)
            break;
          i.fl = k.decode(r, r.uint32());
          continue;
        }
        case 12: {
          if (n !== 98)
            break;
          i.fr = k.decode(r, r.uint32());
          continue;
        }
        case 13: {
          if (n !== 106)
            break;
          i.bl = k.decode(r, r.uint32());
          continue;
        }
        case 14: {
          if (n !== 114)
            break;
          i.br = k.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      t: O(e.t) ? globalThis.Number(e.t) : 0,
      x: O(e.x) ? globalThis.Number(e.x) : 0,
      y: O(e.y) ? globalThis.Number(e.y) : 0,
      heading: O(e.heading) ? globalThis.Number(e.heading) : 0,
      vx: O(e.vx) ? globalThis.Number(e.vx) : 0,
      vy: O(e.vy) ? globalThis.Number(e.vy) : 0,
      omega: O(e.omega) ? globalThis.Number(e.omega) : 0,
      ax: O(e.ax) ? globalThis.Number(e.ax) : 0,
      ay: O(e.ay) ? globalThis.Number(e.ay) : 0,
      alpha: O(e.alpha) ? globalThis.Number(e.alpha) : 0,
      fl: O(e.fl) ? k.fromJSON(e.fl) : void 0,
      fr: O(e.fr) ? k.fromJSON(e.fr) : void 0,
      bl: O(e.bl) ? k.fromJSON(e.bl) : void 0,
      br: O(e.br) ? k.fromJSON(e.br) : void 0
    };
  },
  toJSON(e) {
    const t = {};
    return e.t !== 0 && (t.t = e.t), e.x !== 0 && (t.x = e.x), e.y !== 0 && (t.y = e.y), e.heading !== 0 && (t.heading = e.heading), e.vx !== 0 && (t.vx = e.vx), e.vy !== 0 && (t.vy = e.vy), e.omega !== 0 && (t.omega = e.omega), e.ax !== 0 && (t.ax = e.ax), e.ay !== 0 && (t.ay = e.ay), e.alpha !== 0 && (t.alpha = e.alpha), e.fl !== void 0 && (t.fl = k.toJSON(e.fl)), e.fr !== void 0 && (t.fr = k.toJSON(e.fr)), e.bl !== void 0 && (t.bl = k.toJSON(e.bl)), e.br !== void 0 && (t.br = k.toJSON(e.br)), t;
  },
  create(e) {
    return T.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Ve();
    return t.t = e.t ?? 0, t.x = e.x ?? 0, t.y = e.y ?? 0, t.heading = e.heading ?? 0, t.vx = e.vx ?? 0, t.vy = e.vy ?? 0, t.omega = e.omega ?? 0, t.ax = e.ax ?? 0, t.ay = e.ay ?? 0, t.alpha = e.alpha ?? 0, t.fl = e.fl !== void 0 && e.fl !== null ? k.fromPartial(e.fl) : void 0, t.fr = e.fr !== void 0 && e.fr !== null ? k.fromPartial(e.fr) : void 0, t.bl = e.bl !== void 0 && e.bl !== null ? k.fromPartial(e.bl) : void 0, t.br = e.br !== void 0 && e.br !== null ? k.fromPartial(e.br) : void 0, t;
  }
};
function O(e) {
  return e != null;
}
function Fe() {
  return { samples: [] };
}
const K = {
  encode(e, t = new E()) {
    for (const r of e.samples)
      T.encode(r, t.uint32(10).fork()).join();
    return t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Fe();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.samples.push(T.decode(r, r.uint32()));
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      samples: globalThis.Array.isArray(e?.samples) ? e.samples.map((t) => T.fromJSON(t)) : []
    };
  },
  toJSON(e) {
    const t = {};
    return e.samples?.length && (t.samples = e.samples.map((r) => T.toJSON(r))), t;
  },
  create(e) {
    return K.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Fe();
    return t.samples = e.samples?.map((r) => T.fromPartial(r)) || [], t;
  }
};
function Ue() {
  return { samples: [] };
}
const Y = {
  encode(e, t = new E()) {
    for (const r of e.samples)
      U.encode(r, t.uint32(10).fork()).join();
    return t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Ue();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.samples.push(U.decode(r, r.uint32()));
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      samples: globalThis.Array.isArray(e?.samples) ? e.samples.map((t) => U.fromJSON(t)) : []
    };
  },
  toJSON(e) {
    const t = {};
    return e.samples?.length && (t.samples = e.samples.map((r) => U.toJSON(r))), t;
  },
  create(e) {
    return Y.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Ue();
    return t.samples = e.samples?.map((r) => U.fromPartial(r)) || [], t;
  }
};
function $e() {
  return { trajectory: void 0, splits: [], waypoints: [] };
}
const st = {
  encode(e, t = new E()) {
    switch (e.trajectory?.$case) {
      case "swerve":
        K.encode(e.trajectory.swerve, t.uint32(10).fork()).join();
        break;
      case "differential":
        Y.encode(e.trajectory.differential, t.uint32(18).fork()).join();
        break;
    }
    t.uint32(26).fork();
    for (const r of e.splits)
      t.uint64(r);
    t.join(), t.uint32(34).fork();
    for (const r of e.waypoints)
      t.double(r);
    return t.join(), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = $e();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.trajectory = { $case: "swerve", swerve: K.decode(r, r.uint32()) };
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.trajectory = {
            $case: "differential",
            differential: Y.decode(r, r.uint32())
          };
          continue;
        }
        case 3: {
          if (n === 24) {
            i.splits.push(ze(r.uint64()));
            continue;
          }
          if (n === 26) {
            const s = r.uint32() + r.pos;
            for (; r.pos < s; )
              i.splits.push(ze(r.uint64()));
            continue;
          }
          break;
        }
        case 4: {
          if (n === 33) {
            i.waypoints.push(r.double());
            continue;
          }
          if (n === 34) {
            const s = r.uint32() + r.pos;
            for (; r.pos < s; )
              i.waypoints.push(r.double());
            continue;
          }
          break;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return {
      trajectory: Xe(e.swerve) ? { $case: "swerve", swerve: K.fromJSON(e.swerve) } : Xe(e.differential) ? { $case: "differential", differential: Y.fromJSON(e.differential) } : void 0,
      splits: globalThis.Array.isArray(e?.splits) ? e.splits.map((t) => globalThis.Number(t)) : [],
      waypoints: globalThis.Array.isArray(e?.waypoints) ? e.waypoints.map((t) => globalThis.Number(t)) : []
    };
  },
  toJSON(e) {
    const t = {};
    return e.trajectory?.$case === "swerve" ? t.swerve = K.toJSON(e.trajectory.swerve) : e.trajectory?.$case === "differential" && (t.differential = Y.toJSON(e.trajectory.differential)), e.splits?.length && (t.splits = e.splits.map((r) => Math.round(r))), e.waypoints?.length && (t.waypoints = e.waypoints), t;
  },
  create(e) {
    return st.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = $e();
    switch (e.trajectory?.$case) {
      case "swerve": {
        e.trajectory?.swerve !== void 0 && e.trajectory?.swerve !== null && (t.trajectory = { $case: "swerve", swerve: K.fromPartial(e.trajectory.swerve) });
        break;
      }
      case "differential": {
        e.trajectory?.differential !== void 0 && e.trajectory?.differential !== null && (t.trajectory = {
          $case: "differential",
          differential: Y.fromPartial(e.trajectory.differential)
        });
        break;
      }
    }
    return t.splits = e.splits?.map((r) => r) || [], t.waypoints = e.waypoints?.map((r) => r) || [], t;
  }
};
function ze(e) {
  const t = globalThis.Number(e.toString());
  if (t > globalThis.Number.MAX_SAFE_INTEGER)
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  if (t < globalThis.Number.MIN_SAFE_INTEGER)
    throw new globalThis.Error("Value is smaller than Number.MIN_SAFE_INTEGER");
  return t;
}
function Xe(e) {
  return e != null;
}
function Le() {
  return { name: "" };
}
const ut = {
  encode(e, t = new E()) {
    return e.name !== "" && t.uint32(10).string(e.name), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Le();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.name = r.string();
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return { name: Dt(e.name) ? globalThis.String(e.name) : "" };
  },
  toJSON(e) {
    const t = {};
    return e.name !== "" && (t.name = e.name), t;
  },
  create(e) {
    return ut.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Le();
    return t.name = e.name ?? "", t;
  }
};
function Dt(e) {
  return e != null;
}
const qt = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DifferentialSample: U,
  DifferentialTrajectory: Y,
  DriveType: et,
  ForceVector: k,
  GenerationOutput: st,
  SwerveSample: T,
  SwerveTrajectory: K,
  TrajectoryFile: ut,
  driveTypeFromJSON: Tt,
  driveTypeToJSON: Ot,
  parameters: Bt
}, Symbol.toStringTag, { value: "Module" }));
function Ge() {
  return { sample: void 0 };
}
const te = {
  encode(e, t = new E()) {
    return e.sample !== void 0 && T.encode(e.sample, t.uint32(10).fork()).join(), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = Ge();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.sample = T.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return { sample: ft(e.sample) ? T.fromJSON(e.sample) : void 0 };
  },
  toJSON(e) {
    const t = {};
    return e.sample !== void 0 && (t.sample = T.toJSON(e.sample)), t;
  },
  create(e) {
    return te.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Ge();
    return t.sample = e.sample !== void 0 && e.sample !== null ? T.fromPartial(e.sample) : void 0, t;
  }
};
function qe() {
  return { sample: void 0 };
}
const ue = {
  encode(e, t = new E()) {
    return e.sample !== void 0 && T.encode(e.sample, t.uint32(10).fork()).join(), t;
  },
  decode(e, t) {
    const r = e instanceof g ? e : new g(e), u = t === void 0 ? r.len : r.pos + t, i = qe();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.sample = T.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(e) {
    return { sample: ft(e.sample) ? T.fromJSON(e.sample) : void 0 };
  },
  toJSON(e) {
    const t = {};
    return e.sample !== void 0 && (t.sample = T.toJSON(e.sample)), t;
  },
  create(e) {
    return ue.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = qe();
    return t.sample = e.sample !== void 0 && e.sample !== null ? T.fromPartial(e.sample) : void 0, t;
  }
};
function ft(e) {
  return e != null;
}
const Vt = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  EchoSwerveSampleRequest: te,
  EchoSwerveSampleResponse: ue
}, Symbol.toStringTag, { value: "Module" }));
var j = { exports: {} }, Ft = j.exports, We;
function Ut() {
  return We || (We = 1, (function(e, t) {
    (function(r, u) {
      e.exports = u();
    })(Ft, (function() {
      return r = { 418: function(i, n) {
        (function(s, a) {
          for (var b in a) s[b] = a[b];
        })(n, (function(s) {
          var a = {};
          function b(f) {
            if (a[f]) return a[f].exports;
            var d = a[f] = { i: f, l: !1, exports: {} };
            return s[f].call(d.exports, d, d.exports, b), d.l = !0, d.exports;
          }
          return b.m = s, b.c = a, b.i = function(f) {
            return f;
          }, b.d = function(f, d, y) {
            b.o(f, d) || Object.defineProperty(f, d, { configurable: !1, enumerable: !0, get: y });
          }, b.n = function(f) {
            var d = f && f.__esModule ? function() {
              return f.default;
            } : function() {
              return f;
            };
            return b.d(d, "a", d), d;
          }, b.o = function(f, d) {
            return Object.prototype.hasOwnProperty.call(f, d);
          }, b.p = "", b(b.s = 1);
        })([function(s, a, b) {
          Object.defineProperty(a, "__esModule", { value: !0 });
          var f = b(3), d = (function() {
            function y(l, h) {
              l === void 0 && (l = {}), h === void 0 && (h = { splitValues: !1 });
              var p, v = this;
              this.headersMap = {}, l && (typeof Headers < "u" && l instanceof Headers ? f.getHeaderKeys(l).forEach((function(o) {
                f.getHeaderValues(l, o).forEach((function(c) {
                  h.splitValues ? v.append(o, f.splitHeaderValue(c)) : v.append(o, c);
                }));
              })) : typeof (p = l) == "object" && typeof p.headersMap == "object" && typeof p.forEach == "function" ? l.forEach((function(o, c) {
                v.append(o, c);
              })) : typeof Map < "u" && l instanceof Map ? l.forEach((function(o, c) {
                v.append(c, o);
              })) : typeof l == "string" ? this.appendFromString(l) : typeof l == "object" && Object.getOwnPropertyNames(l).forEach((function(o) {
                var c = l[o];
                Array.isArray(c) ? c.forEach((function(m) {
                  v.append(o, m);
                })) : v.append(o, c);
              })));
            }
            return y.prototype.appendFromString = function(l) {
              for (var h = l.split(`\r
`), p = 0; p < h.length; p++) {
                var v = h[p], o = v.indexOf(":");
                if (o > 0) {
                  var c = v.substring(0, o).trim(), m = v.substring(o + 1).trim();
                  this.append(c, m);
                }
              }
            }, y.prototype.delete = function(l, h) {
              var p = f.normalizeName(l);
              if (h === void 0) delete this.headersMap[p];
              else {
                var v = this.headersMap[p];
                if (v) {
                  var o = v.indexOf(h);
                  o >= 0 && v.splice(o, 1), v.length === 0 && delete this.headersMap[p];
                }
              }
            }, y.prototype.append = function(l, h) {
              var p = this, v = f.normalizeName(l);
              Array.isArray(this.headersMap[v]) || (this.headersMap[v] = []), Array.isArray(h) ? h.forEach((function(o) {
                p.headersMap[v].push(f.normalizeValue(o));
              })) : this.headersMap[v].push(f.normalizeValue(h));
            }, y.prototype.set = function(l, h) {
              var p = f.normalizeName(l);
              if (Array.isArray(h)) {
                var v = [];
                h.forEach((function(o) {
                  v.push(f.normalizeValue(o));
                })), this.headersMap[p] = v;
              } else this.headersMap[p] = [f.normalizeValue(h)];
            }, y.prototype.has = function(l, h) {
              var p = this.headersMap[f.normalizeName(l)];
              if (!Array.isArray(p)) return !1;
              if (h !== void 0) {
                var v = f.normalizeValue(h);
                return p.indexOf(v) >= 0;
              }
              return !0;
            }, y.prototype.get = function(l) {
              var h = this.headersMap[f.normalizeName(l)];
              return h !== void 0 ? h.concat() : [];
            }, y.prototype.forEach = function(l) {
              var h = this;
              Object.getOwnPropertyNames(this.headersMap).forEach((function(p) {
                l(p, h.headersMap[p]);
              }), this);
            }, y.prototype.toHeaders = function() {
              if (typeof Headers < "u") {
                var l = new Headers();
                return this.forEach((function(h, p) {
                  p.forEach((function(v) {
                    l.append(h, v);
                  }));
                })), l;
              }
              throw new Error("Headers class is not defined");
            }, y;
          })();
          a.BrowserHeaders = d;
        }, function(s, a, b) {
          Object.defineProperty(a, "__esModule", { value: !0 });
          var f = b(0);
          a.BrowserHeaders = f.BrowserHeaders;
        }, function(s, a, b) {
          Object.defineProperty(a, "__esModule", { value: !0 }), a.iterateHeaders = function(f, d) {
            for (var y = f[Symbol.iterator](), l = y.next(); !l.done; ) d(l.value[0]), l = y.next();
          }, a.iterateHeadersKeys = function(f, d) {
            for (var y = f.keys(), l = y.next(); !l.done; ) d(l.value), l = y.next();
          };
        }, function(s, a, b) {
          Object.defineProperty(a, "__esModule", { value: !0 });
          var f = b(2);
          a.normalizeName = function(d) {
            if (typeof d != "string" && (d = String(d)), /[^a-z0-9\-#$%&'*+.\^_`|~]/i.test(d)) throw new TypeError("Invalid character in header field name");
            return d.toLowerCase();
          }, a.normalizeValue = function(d) {
            return typeof d != "string" && (d = String(d)), d;
          }, a.getHeaderValues = function(d, y) {
            var l = d;
            if (l instanceof Headers && l.getAll) return l.getAll(y);
            var h = l.get(y);
            return h && typeof h == "string" ? [h] : h;
          }, a.getHeaderKeys = function(d) {
            var y = d, l = {}, h = [];
            return y.keys ? f.iterateHeadersKeys(y, (function(p) {
              l[p] || (l[p] = !0, h.push(p));
            })) : y.forEach ? y.forEach((function(p, v) {
              l[v] || (l[v] = !0, h.push(v));
            })) : f.iterateHeaders(y, (function(p) {
              var v = p[0];
              l[v] || (l[v] = !0, h.push(v));
            })), h;
          }, a.splitHeaderValue = function(d) {
            var y = [];
            return d.split(", ").forEach((function(l) {
              l.split(",").forEach((function(h) {
                y.push(h);
              }));
            })), y;
          };
        }]));
      }, 617: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.ChunkParser = n.ChunkType = n.encodeASCII = n.decodeASCII = void 0;
        var a, b = s(65);
        function f(o) {
          return (c = o) === 9 || c === 10 || c === 13 || o >= 32 && o <= 126;
          var c;
        }
        function d(o) {
          for (var c = 0; c !== o.length; ++c) if (!f(o[c])) throw new Error("Metadata is not valid (printable) ASCII");
          return String.fromCharCode.apply(String, Array.prototype.slice.call(o));
        }
        function y(o) {
          return (128 & o.getUint8(0)) == 128;
        }
        function l(o) {
          return o.getUint32(1, !1);
        }
        function h(o, c, m) {
          return o.byteLength - c >= m;
        }
        function p(o, c, m) {
          if (o.slice) return o.slice(c, m);
          var x = o.length;
          m !== void 0 && (x = m);
          for (var w = new Uint8Array(x - c), _ = 0, D = c; D < x; D++) w[_++] = o[D];
          return w;
        }
        n.decodeASCII = d, n.encodeASCII = function(o) {
          for (var c = new Uint8Array(o.length), m = 0; m !== o.length; ++m) {
            var x = o.charCodeAt(m);
            if (!f(x)) throw new Error("Metadata contains invalid ASCII");
            c[m] = x;
          }
          return c;
        }, (function(o) {
          o[o.MESSAGE = 1] = "MESSAGE", o[o.TRAILERS = 2] = "TRAILERS";
        })(a = n.ChunkType || (n.ChunkType = {}));
        var v = (function() {
          function o() {
            this.buffer = null, this.position = 0;
          }
          return o.prototype.parse = function(c, m) {
            if (c.length === 0 && m) return [];
            var x, w = [];
            if (this.buffer == null) this.buffer = c, this.position = 0;
            else if (this.position === this.buffer.byteLength) this.buffer = c, this.position = 0;
            else {
              var _ = this.buffer.byteLength - this.position, D = new Uint8Array(_ + c.byteLength), ht = p(this.buffer, this.position);
              D.set(ht, 0);
              var pt = new Uint8Array(c);
              D.set(pt, _), this.buffer = D, this.position = 0;
            }
            for (; ; ) {
              if (!h(this.buffer, this.position, 5)) return w;
              var re = p(this.buffer, this.position, this.position + 5), fe = new DataView(re.buffer, re.byteOffset, re.byteLength), ne = l(fe);
              if (!h(this.buffer, this.position, 5 + ne)) return w;
              var le = p(this.buffer, this.position + 5, this.position + 5 + ne);
              if (this.position += 5 + ne, y(fe)) return w.push({ chunkType: a.TRAILERS, trailers: (x = le, new b.Metadata(d(x))) }), w;
              w.push({ chunkType: a.MESSAGE, data: le });
            }
          }, o;
        })();
        n.ChunkParser = v;
      }, 8: function(i, n) {
        var s;
        Object.defineProperty(n, "__esModule", { value: !0 }), n.httpStatusToCode = n.Code = void 0, (function(a) {
          a[a.OK = 0] = "OK", a[a.Canceled = 1] = "Canceled", a[a.Unknown = 2] = "Unknown", a[a.InvalidArgument = 3] = "InvalidArgument", a[a.DeadlineExceeded = 4] = "DeadlineExceeded", a[a.NotFound = 5] = "NotFound", a[a.AlreadyExists = 6] = "AlreadyExists", a[a.PermissionDenied = 7] = "PermissionDenied", a[a.ResourceExhausted = 8] = "ResourceExhausted", a[a.FailedPrecondition = 9] = "FailedPrecondition", a[a.Aborted = 10] = "Aborted", a[a.OutOfRange = 11] = "OutOfRange", a[a.Unimplemented = 12] = "Unimplemented", a[a.Internal = 13] = "Internal", a[a.Unavailable = 14] = "Unavailable", a[a.DataLoss = 15] = "DataLoss", a[a.Unauthenticated = 16] = "Unauthenticated";
        })(s = n.Code || (n.Code = {})), n.httpStatusToCode = function(a) {
          switch (a) {
            case 0:
              return s.Internal;
            case 200:
              return s.OK;
            case 400:
              return s.InvalidArgument;
            case 401:
              return s.Unauthenticated;
            case 403:
              return s.PermissionDenied;
            case 404:
              return s.NotFound;
            case 409:
              return s.Aborted;
            case 412:
              return s.FailedPrecondition;
            case 429:
              return s.ResourceExhausted;
            case 499:
              return s.Canceled;
            case 500:
              return s.Unknown;
            case 501:
              return s.Unimplemented;
            case 503:
              return s.Unavailable;
            case 504:
              return s.DeadlineExceeded;
            default:
              return s.Unknown;
          }
        };
      }, 934: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.client = void 0;
        var a = s(65), b = s(617), f = s(8), d = s(346), y = s(57), l = s(882);
        n.client = function(v, o) {
          return new h(v, o);
        };
        var h = (function() {
          function v(o, c) {
            this.started = !1, this.sentFirstMessage = !1, this.completed = !1, this.closed = !1, this.finishedSending = !1, this.onHeadersCallbacks = [], this.onMessageCallbacks = [], this.onEndCallbacks = [], this.parser = new b.ChunkParser(), this.methodDefinition = o, this.props = c, this.createTransport();
          }
          return v.prototype.createTransport = function() {
            var o = this.props.host + "/" + this.methodDefinition.service.serviceName + "/" + this.methodDefinition.methodName, c = { methodDefinition: this.methodDefinition, debug: this.props.debug || !1, url: o, onHeaders: this.onTransportHeaders.bind(this), onChunk: this.onTransportChunk.bind(this), onEnd: this.onTransportEnd.bind(this) };
            this.props.transport ? this.transport = this.props.transport(c) : this.transport = y.makeDefaultTransport(c);
          }, v.prototype.onTransportHeaders = function(o, c) {
            if (this.props.debug && d.debug("onHeaders", o, c), this.closed) this.props.debug && d.debug("grpc.onHeaders received after request was closed - ignoring");
            else if (c !== 0) {
              this.responseHeaders = o, this.props.debug && d.debug("onHeaders.responseHeaders", JSON.stringify(this.responseHeaders, null, 2));
              var m = p(o);
              this.props.debug && d.debug("onHeaders.gRPCStatus", m);
              var x = m && m >= 0 ? m : f.httpStatusToCode(c);
              this.props.debug && d.debug("onHeaders.code", x);
              var w = o.get("grpc-message") || [];
              if (this.props.debug && d.debug("onHeaders.gRPCMessage", w), this.rawOnHeaders(o), x !== f.Code.OK) {
                var _ = this.decodeGRPCStatus(w[0]);
                this.rawOnError(x, _, o);
              }
            }
          }, v.prototype.onTransportChunk = function(o) {
            var c = this;
            if (this.closed) this.props.debug && d.debug("grpc.onChunk received after request was closed - ignoring");
            else {
              var m = [];
              try {
                m = this.parser.parse(o);
              } catch (x) {
                return this.props.debug && d.debug("onChunk.parsing error", x, x.message), void this.rawOnError(f.Code.Internal, "parsing error: " + x.message);
              }
              m.forEach((function(x) {
                if (x.chunkType === b.ChunkType.MESSAGE) {
                  var w = c.methodDefinition.responseType.deserializeBinary(x.data);
                  c.rawOnMessage(w);
                } else x.chunkType === b.ChunkType.TRAILERS && (c.responseHeaders ? (c.responseTrailers = new a.Metadata(x.trailers), c.props.debug && d.debug("onChunk.trailers", c.responseTrailers)) : (c.responseHeaders = new a.Metadata(x.trailers), c.rawOnHeaders(c.responseHeaders)));
              }));
            }
          }, v.prototype.onTransportEnd = function() {
            if (this.props.debug && d.debug("grpc.onEnd"), this.closed) this.props.debug && d.debug("grpc.onEnd received after request was closed - ignoring");
            else if (this.responseTrailers !== void 0) {
              var o = p(this.responseTrailers);
              if (o !== null) {
                var c = this.responseTrailers.get("grpc-message"), m = this.decodeGRPCStatus(c[0]);
                this.rawOnEnd(o, m, this.responseTrailers);
              } else this.rawOnError(f.Code.Internal, "Response closed without grpc-status (Trailers provided)");
            } else {
              if (this.responseHeaders === void 0) return void this.rawOnError(f.Code.Unknown, "Response closed without headers");
              var x = p(this.responseHeaders), w = this.responseHeaders.get("grpc-message");
              if (this.props.debug && d.debug("grpc.headers only response ", x, w), x === null) return void this.rawOnEnd(f.Code.Unknown, "Response closed without grpc-status (Headers only)", this.responseHeaders);
              var _ = this.decodeGRPCStatus(w[0]);
              this.rawOnEnd(x, _, this.responseHeaders);
            }
          }, v.prototype.decodeGRPCStatus = function(o) {
            if (!o) return "";
            try {
              return decodeURIComponent(o);
            } catch {
              return o;
            }
          }, v.prototype.rawOnEnd = function(o, c, m) {
            var x = this;
            this.props.debug && d.debug("rawOnEnd", o, c, m), this.completed || (this.completed = !0, this.onEndCallbacks.forEach((function(w) {
              if (!x.closed) try {
                w(o, c, m);
              } catch (_) {
                setTimeout((function() {
                  throw _;
                }), 0);
              }
            })));
          }, v.prototype.rawOnHeaders = function(o) {
            this.props.debug && d.debug("rawOnHeaders", o), this.completed || this.onHeadersCallbacks.forEach((function(c) {
              try {
                c(o);
              } catch (m) {
                setTimeout((function() {
                  throw m;
                }), 0);
              }
            }));
          }, v.prototype.rawOnError = function(o, c, m) {
            var x = this;
            m === void 0 && (m = new a.Metadata()), this.props.debug && d.debug("rawOnError", o, c), this.completed || (this.completed = !0, this.onEndCallbacks.forEach((function(w) {
              if (!x.closed) try {
                w(o, c, m);
              } catch (_) {
                setTimeout((function() {
                  throw _;
                }), 0);
              }
            })));
          }, v.prototype.rawOnMessage = function(o) {
            var c = this;
            this.props.debug && d.debug("rawOnMessage", o.toObject()), this.completed || this.closed || this.onMessageCallbacks.forEach((function(m) {
              if (!c.closed) try {
                m(o);
              } catch (x) {
                setTimeout((function() {
                  throw x;
                }), 0);
              }
            }));
          }, v.prototype.onHeaders = function(o) {
            this.onHeadersCallbacks.push(o);
          }, v.prototype.onMessage = function(o) {
            this.onMessageCallbacks.push(o);
          }, v.prototype.onEnd = function(o) {
            this.onEndCallbacks.push(o);
          }, v.prototype.start = function(o) {
            if (this.started) throw new Error("Client already started - cannot .start()");
            this.started = !0;
            var c = new a.Metadata(o || {});
            c.set("content-type", "application/grpc-web+proto"), c.set("x-grpc-web", "1"), this.transport.start(c);
          }, v.prototype.send = function(o) {
            if (!this.started) throw new Error("Client not started - .start() must be called before .send()");
            if (this.closed) throw new Error("Client already closed - cannot .send()");
            if (this.finishedSending) throw new Error("Client already finished sending - cannot .send()");
            if (!this.methodDefinition.requestStream && this.sentFirstMessage) throw new Error("Message already sent for non-client-streaming method - cannot .send()");
            this.sentFirstMessage = !0;
            var c = l.frameRequest(o);
            this.transport.sendMessage(c);
          }, v.prototype.finishSend = function() {
            if (!this.started) throw new Error("Client not started - .finishSend() must be called before .close()");
            if (this.closed) throw new Error("Client already closed - cannot .send()");
            if (this.finishedSending) throw new Error("Client already finished sending - cannot .finishSend()");
            this.finishedSending = !0, this.transport.finishSend();
          }, v.prototype.close = function() {
            if (!this.started) throw new Error("Client not started - .start() must be called before .close()");
            if (this.closed) throw new Error("Client already closed - cannot .close()");
            this.closed = !0, this.props.debug && d.debug("request.abort aborting request"), this.transport.cancel();
          }, v;
        })();
        function p(v) {
          var o = v.get("grpc-status") || [];
          if (o.length > 0) try {
            var c = o[0];
            return parseInt(c, 10);
          } catch {
            return null;
          }
          return null;
        }
      }, 346: function(i, n) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.debug = void 0, n.debug = function() {
          for (var s = [], a = 0; a < arguments.length; a++) s[a] = arguments[a];
          console.debug ? console.debug.apply(null, s) : console.log.apply(null, s);
        };
      }, 607: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.grpc = void 0;
        var a, b = s(418), f = s(57), d = s(229), y = s(540), l = s(210), h = s(859), p = s(8), v = s(938), o = s(35), c = s(934);
        (a = n.grpc || (n.grpc = {})).setDefaultTransport = f.setDefaultTransportFactory, a.CrossBrowserHttpTransport = h.CrossBrowserHttpTransport, a.FetchReadableStreamTransport = d.FetchReadableStreamTransport, a.XhrTransport = l.XhrTransport, a.WebsocketTransport = y.WebsocketTransport, a.Code = p.Code, a.Metadata = b.BrowserHeaders, a.client = function(m, x) {
          return c.client(m, x);
        }, a.invoke = v.invoke, a.unary = o.unary;
      }, 938: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.invoke = void 0;
        var a = s(934);
        n.invoke = function(b, f) {
          if (b.requestStream) throw new Error(".invoke cannot be used with client-streaming methods. Use .client instead.");
          var d = a.client(b, { host: f.host, transport: f.transport, debug: f.debug });
          return f.onHeaders && d.onHeaders(f.onHeaders), f.onMessage && d.onMessage(f.onMessage), f.onEnd && d.onEnd(f.onEnd), d.start(f.metadata), d.send(f.request), d.finishSend(), { close: function() {
            d.close();
          } };
        };
      }, 65: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.Metadata = void 0;
        var a = s(418);
        Object.defineProperty(n, "Metadata", { enumerable: !0, get: function() {
          return a.BrowserHeaders;
        } });
      }, 57: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.makeDefaultTransport = n.setDefaultTransportFactory = void 0;
        var a = s(859), b = function(f) {
          return a.CrossBrowserHttpTransport({ withCredentials: !1 })(f);
        };
        n.setDefaultTransportFactory = function(f) {
          b = f;
        }, n.makeDefaultTransport = function(f) {
          return b(f);
        };
      }, 229: function(i, n, s) {
        var a = this && this.__assign || function() {
          return (a = Object.assign || function(y) {
            for (var l, h = 1, p = arguments.length; h < p; h++) for (var v in l = arguments[h]) Object.prototype.hasOwnProperty.call(l, v) && (y[v] = l[v]);
            return y;
          }).apply(this, arguments);
        };
        Object.defineProperty(n, "__esModule", { value: !0 }), n.detectFetchSupport = n.FetchReadableStreamTransport = void 0;
        var b = s(65), f = s(346);
        n.FetchReadableStreamTransport = function(y) {
          return function(l) {
            return (function(h, p) {
              return h.debug && f.debug("fetchRequest", h), new d(h, p);
            })(l, y);
          };
        };
        var d = (function() {
          function y(l, h) {
            this.cancelled = !1, this.controller = self.AbortController && new AbortController(), this.options = l, this.init = h;
          }
          return y.prototype.pump = function(l, h) {
            var p = this;
            if (this.reader = l, this.cancelled) return this.options.debug && f.debug("Fetch.pump.cancel at first pump"), void this.reader.cancel().catch((function(v) {
              p.options.debug && f.debug("Fetch.pump.reader.cancel exception", v);
            }));
            this.reader.read().then((function(v) {
              if (v.done) return p.options.onEnd(), h;
              p.options.onChunk(v.value), p.pump(p.reader, h);
            })).catch((function(v) {
              p.cancelled ? p.options.debug && f.debug("Fetch.catch - request cancelled") : (p.cancelled = !0, p.options.debug && f.debug("Fetch.catch", v.message), p.options.onEnd(v));
            }));
          }, y.prototype.send = function(l) {
            var h = this;
            fetch(this.options.url, a(a({}, this.init), { headers: this.metadata.toHeaders(), method: "POST", body: l, signal: this.controller && this.controller.signal })).then((function(p) {
              if (h.options.debug && f.debug("Fetch.response", p), h.options.onHeaders(new b.Metadata(p.headers), p.status), !p.body) return p;
              h.pump(p.body.getReader(), p);
            })).catch((function(p) {
              h.cancelled ? h.options.debug && f.debug("Fetch.catch - request cancelled") : (h.cancelled = !0, h.options.debug && f.debug("Fetch.catch", p.message), h.options.onEnd(p));
            }));
          }, y.prototype.sendMessage = function(l) {
            this.send(l);
          }, y.prototype.finishSend = function() {
          }, y.prototype.start = function(l) {
            this.metadata = l;
          }, y.prototype.cancel = function() {
            var l = this;
            this.cancelled ? this.options.debug && f.debug("Fetch.cancel already cancelled") : (this.cancelled = !0, this.controller ? (this.options.debug && f.debug("Fetch.cancel.controller.abort"), this.controller.abort()) : this.options.debug && f.debug("Fetch.cancel.missing abort controller"), this.reader ? (this.options.debug && f.debug("Fetch.cancel.reader.cancel"), this.reader.cancel().catch((function(h) {
              l.options.debug && f.debug("Fetch.cancel.reader.cancel exception", h);
            }))) : this.options.debug && f.debug("Fetch.cancel before reader"));
          }, y;
        })();
        n.detectFetchSupport = function() {
          return typeof Response < "u" && Response.prototype.hasOwnProperty("body") && typeof Headers == "function";
        };
      }, 859: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.CrossBrowserHttpTransport = void 0;
        var a = s(229), b = s(210);
        n.CrossBrowserHttpTransport = function(f) {
          if (a.detectFetchSupport()) {
            var d = { credentials: f.withCredentials ? "include" : "same-origin" };
            return a.FetchReadableStreamTransport(d);
          }
          return b.XhrTransport({ withCredentials: f.withCredentials });
        };
      }, 210: function(i, n, s) {
        var a, b = this && this.__extends || (a = function(o, c) {
          return (a = Object.setPrototypeOf || { __proto__: [] } instanceof Array && function(m, x) {
            m.__proto__ = x;
          } || function(m, x) {
            for (var w in x) Object.prototype.hasOwnProperty.call(x, w) && (m[w] = x[w]);
          })(o, c);
        }, function(o, c) {
          function m() {
            this.constructor = o;
          }
          a(o, c), o.prototype = c === null ? Object.create(c) : (m.prototype = c.prototype, new m());
        });
        Object.defineProperty(n, "__esModule", { value: !0 }), n.stringToArrayBuffer = n.MozChunkedArrayBufferXHR = n.XHR = n.XhrTransport = void 0;
        var f = s(65), d = s(346), y = s(849);
        n.XhrTransport = function(o) {
          return function(c) {
            if (y.detectMozXHRSupport()) return new h(c, o);
            if (y.detectXHROverrideMimeTypeSupport()) return new l(c, o);
            throw new Error("This environment's XHR implementation cannot support binary transfer.");
          };
        };
        var l = (function() {
          function o(c, m) {
            this.options = c, this.init = m;
          }
          return o.prototype.onProgressEvent = function() {
            this.options.debug && d.debug("XHR.onProgressEvent.length: ", this.xhr.response.length);
            var c = this.xhr.response.substr(this.index);
            this.index = this.xhr.response.length;
            var m = v(c);
            this.options.onChunk(m);
          }, o.prototype.onLoadEvent = function() {
            this.options.debug && d.debug("XHR.onLoadEvent"), this.options.onEnd();
          }, o.prototype.onStateChange = function() {
            this.options.debug && d.debug("XHR.onStateChange", this.xhr.readyState), this.xhr.readyState === XMLHttpRequest.HEADERS_RECEIVED && this.options.onHeaders(new f.Metadata(this.xhr.getAllResponseHeaders()), this.xhr.status);
          }, o.prototype.sendMessage = function(c) {
            this.xhr.send(c);
          }, o.prototype.finishSend = function() {
          }, o.prototype.start = function(c) {
            var m = this;
            this.metadata = c;
            var x = new XMLHttpRequest();
            this.xhr = x, x.open("POST", this.options.url), this.configureXhr(), this.metadata.forEach((function(w, _) {
              x.setRequestHeader(w, _.join(", "));
            })), x.withCredentials = !!this.init.withCredentials, x.addEventListener("readystatechange", this.onStateChange.bind(this)), x.addEventListener("progress", this.onProgressEvent.bind(this)), x.addEventListener("loadend", this.onLoadEvent.bind(this)), x.addEventListener("error", (function(w) {
              m.options.debug && d.debug("XHR.error", w), m.options.onEnd(w.error);
            }));
          }, o.prototype.configureXhr = function() {
            this.xhr.responseType = "text", this.xhr.overrideMimeType("text/plain; charset=x-user-defined");
          }, o.prototype.cancel = function() {
            this.options.debug && d.debug("XHR.abort"), this.xhr.abort();
          }, o;
        })();
        n.XHR = l;
        var h = (function(o) {
          function c() {
            return o !== null && o.apply(this, arguments) || this;
          }
          return b(c, o), c.prototype.configureXhr = function() {
            this.options.debug && d.debug("MozXHR.configureXhr: setting responseType to 'moz-chunked-arraybuffer'"), this.xhr.responseType = "moz-chunked-arraybuffer";
          }, c.prototype.onProgressEvent = function() {
            var m = this.xhr.response;
            this.options.debug && d.debug("MozXHR.onProgressEvent: ", new Uint8Array(m)), this.options.onChunk(new Uint8Array(m));
          }, c;
        })(l);
        function p(o, c) {
          var m = o.charCodeAt(c);
          if (m >= 55296 && m <= 56319) {
            var x = o.charCodeAt(c + 1);
            x >= 56320 && x <= 57343 && (m = 65536 + (m - 55296 << 10) + (x - 56320));
          }
          return m;
        }
        function v(o) {
          for (var c = new Uint8Array(o.length), m = 0, x = 0; x < o.length; x++) {
            var w = String.prototype.codePointAt ? o.codePointAt(x) : p(o, x);
            c[m++] = 255 & w;
          }
          return c;
        }
        n.MozChunkedArrayBufferXHR = h, n.stringToArrayBuffer = v;
      }, 849: function(i, n) {
        var s;
        function a() {
          if (s !== void 0) return s;
          if (XMLHttpRequest) {
            s = new XMLHttpRequest();
            try {
              s.open("GET", "https://localhost");
            } catch {
            }
          }
          return s;
        }
        function b(f) {
          var d = a();
          if (!d) return !1;
          try {
            return d.responseType = f, d.responseType === f;
          } catch {
          }
          return !1;
        }
        Object.defineProperty(n, "__esModule", { value: !0 }), n.detectXHROverrideMimeTypeSupport = n.detectMozXHRSupport = n.xhrSupportsResponseType = void 0, n.xhrSupportsResponseType = b, n.detectMozXHRSupport = function() {
          return typeof XMLHttpRequest < "u" && b("moz-chunked-arraybuffer");
        }, n.detectXHROverrideMimeTypeSupport = function() {
          return typeof XMLHttpRequest < "u" && XMLHttpRequest.prototype.hasOwnProperty("overrideMimeType");
        };
      }, 540: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.WebsocketTransport = void 0;
        var a, b = s(346), f = s(617);
        (function(y) {
          y[y.FINISH_SEND = 1] = "FINISH_SEND";
        })(a || (a = {}));
        var d = new Uint8Array([1]);
        n.WebsocketTransport = function() {
          return function(y) {
            return (function(l) {
              l.debug && b.debug("websocketRequest", l);
              var h, p = (function(c) {
                if (c.substr(0, 8) === "https://") return "wss://" + c.substr(8);
                if (c.substr(0, 7) === "http://") return "ws://" + c.substr(7);
                throw new Error("Websocket transport constructed with non-https:// or http:// host.");
              })(l.url), v = [];
              function o(c) {
                if (c === a.FINISH_SEND) h.send(d);
                else {
                  var m = c, x = new Int8Array(m.byteLength + 1);
                  x.set(new Uint8Array([0])), x.set(m, 1), h.send(x);
                }
              }
              return { sendMessage: function(c) {
                h && h.readyState !== h.CONNECTING ? o(c) : v.push(c);
              }, finishSend: function() {
                h && h.readyState !== h.CONNECTING ? o(a.FINISH_SEND) : v.push(a.FINISH_SEND);
              }, start: function(c) {
                (h = new WebSocket(p, ["grpc-websockets"])).binaryType = "arraybuffer", h.onopen = function() {
                  var m;
                  l.debug && b.debug("websocketRequest.onopen"), h.send((m = "", c.forEach((function(x, w) {
                    m += x + ": " + w.join(", ") + `\r
`;
                  })), f.encodeASCII(m))), v.forEach((function(x) {
                    o(x);
                  }));
                }, h.onclose = function(m) {
                  l.debug && b.debug("websocketRequest.onclose", m), l.onEnd();
                }, h.onerror = function(m) {
                  l.debug && b.debug("websocketRequest.onerror", m);
                }, h.onmessage = function(m) {
                  l.onChunk(new Uint8Array(m.data));
                };
              }, cancel: function() {
                l.debug && b.debug("websocket.abort"), h.close();
              } };
            })(y);
          };
        };
      }, 35: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.unary = void 0;
        var a = s(65), b = s(934);
        n.unary = function(f, d) {
          if (f.responseStream) throw new Error(".unary cannot be used with server-streaming methods. Use .invoke or .client instead.");
          if (f.requestStream) throw new Error(".unary cannot be used with client-streaming methods. Use .client instead.");
          var y = null, l = null, h = b.client(f, { host: d.host, transport: d.transport, debug: d.debug });
          return h.onHeaders((function(p) {
            y = p;
          })), h.onMessage((function(p) {
            l = p;
          })), h.onEnd((function(p, v, o) {
            d.onEnd({ status: p, statusMessage: v, headers: y || new a.Metadata(), message: l, trailers: o });
          })), h.start(d.metadata), h.send(d.request), h.finishSend(), { close: function() {
            h.close();
          } };
        };
      }, 882: function(i, n) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.frameRequest = void 0, n.frameRequest = function(s) {
          var a = s.serializeBinary(), b = new ArrayBuffer(a.byteLength + 5);
          return new DataView(b, 1, 4).setUint32(0, a.length, !1), new Uint8Array(b, 5).set(a), new Uint8Array(b);
        };
      } }, u = {}, (function i(n) {
        if (u[n]) return u[n].exports;
        var s = u[n] = { exports: {} };
        return r[n].call(s.exports, s, s.exports, i), s.exports;
      })(607);
      var r, u;
    }));
  })(j)), j.exports;
}
var Ke = Ut(), ee = { exports: {} }, $t = ee.exports, Ye;
function zt() {
  return Ye || (Ye = 1, (function(e, t) {
    (function(u, i) {
      e.exports = i();
    })($t, function() {
      return (
        /******/
        (function(r) {
          var u = {};
          function i(n) {
            if (u[n])
              return u[n].exports;
            var s = u[n] = {
              /******/
              i: n,
              /******/
              l: !1,
              /******/
              exports: {}
              /******/
            };
            return r[n].call(s.exports, s, s.exports, i), s.l = !0, s.exports;
          }
          return i.m = r, i.c = u, i.i = function(n) {
            return n;
          }, i.d = function(n, s, a) {
            i.o(n, s) || Object.defineProperty(n, s, {
              /******/
              configurable: !1,
              /******/
              enumerable: !0,
              /******/
              get: a
              /******/
            });
          }, i.n = function(n) {
            var s = n && n.__esModule ? (
              /******/
              function() {
                return n.default;
              }
            ) : (
              /******/
              function() {
                return n;
              }
            );
            return i.d(s, "a", s), s;
          }, i.o = function(n, s) {
            return Object.prototype.hasOwnProperty.call(n, s);
          }, i.p = "", i(i.s = 1);
        })([
          /* 0 */
          /***/
          (function(r, u, i) {
            Object.defineProperty(u, "__esModule", { value: !0 });
            var n = i(3);
            function s(b) {
              return typeof b == "object" && typeof b.headersMap == "object" && typeof b.forEach == "function";
            }
            var a = (function() {
              function b(f, d) {
                f === void 0 && (f = {}), d === void 0 && (d = { splitValues: !1 });
                var y = this;
                if (this.headersMap = {}, f)
                  if (typeof Headers < "u" && f instanceof Headers) {
                    var l = n.getHeaderKeys(f);
                    l.forEach(function(p) {
                      var v = n.getHeaderValues(f, p);
                      v.forEach(function(o) {
                        d.splitValues ? y.append(p, n.splitHeaderValue(o)) : y.append(p, o);
                      });
                    });
                  } else if (s(f))
                    f.forEach(function(p, v) {
                      y.append(p, v);
                    });
                  else if (typeof Map < "u" && f instanceof Map) {
                    var h = f;
                    h.forEach(function(p, v) {
                      y.append(v, p);
                    });
                  } else typeof f == "string" ? this.appendFromString(f) : typeof f == "object" && Object.getOwnPropertyNames(f).forEach(function(p) {
                    var v = f, o = v[p];
                    Array.isArray(o) ? o.forEach(function(c) {
                      y.append(p, c);
                    }) : y.append(p, o);
                  });
              }
              return b.prototype.appendFromString = function(f) {
                for (var d = f.split(`\r
`), y = 0; y < d.length; y++) {
                  var l = d[y], h = l.indexOf(":");
                  if (h > 0) {
                    var p = l.substring(0, h).trim(), v = l.substring(h + 1).trim();
                    this.append(p, v);
                  }
                }
              }, b.prototype.delete = function(f, d) {
                var y = n.normalizeName(f);
                if (d === void 0)
                  delete this.headersMap[y];
                else {
                  var l = this.headersMap[y];
                  if (l) {
                    var h = l.indexOf(d);
                    h >= 0 && l.splice(h, 1), l.length === 0 && delete this.headersMap[y];
                  }
                }
              }, b.prototype.append = function(f, d) {
                var y = this, l = n.normalizeName(f);
                Array.isArray(this.headersMap[l]) || (this.headersMap[l] = []), Array.isArray(d) ? d.forEach(function(h) {
                  y.headersMap[l].push(n.normalizeValue(h));
                }) : this.headersMap[l].push(n.normalizeValue(d));
              }, b.prototype.set = function(f, d) {
                var y = n.normalizeName(f);
                if (Array.isArray(d)) {
                  var l = [];
                  d.forEach(function(h) {
                    l.push(n.normalizeValue(h));
                  }), this.headersMap[y] = l;
                } else
                  this.headersMap[y] = [n.normalizeValue(d)];
              }, b.prototype.has = function(f, d) {
                var y = this.headersMap[n.normalizeName(f)], l = Array.isArray(y);
                if (!l)
                  return !1;
                if (d !== void 0) {
                  var h = n.normalizeValue(d);
                  return y.indexOf(h) >= 0;
                } else
                  return !0;
              }, b.prototype.get = function(f) {
                var d = this.headersMap[n.normalizeName(f)];
                return d !== void 0 ? d.concat() : [];
              }, b.prototype.forEach = function(f) {
                var d = this;
                Object.getOwnPropertyNames(this.headersMap).forEach(function(y) {
                  f(y, d.headersMap[y]);
                }, this);
              }, b.prototype.toHeaders = function() {
                if (typeof Headers < "u") {
                  var f = new Headers();
                  return this.forEach(function(d, y) {
                    y.forEach(function(l) {
                      f.append(d, l);
                    });
                  }), f;
                } else
                  throw new Error("Headers class is not defined");
              }, b;
            })();
            u.BrowserHeaders = a;
          }),
          /* 1 */
          /***/
          (function(r, u, i) {
            Object.defineProperty(u, "__esModule", { value: !0 });
            var n = i(0);
            u.BrowserHeaders = n.BrowserHeaders;
          }),
          /* 2 */
          /***/
          (function(r, u, i) {
            Object.defineProperty(u, "__esModule", { value: !0 });
            function n(a, b) {
              for (var f = a[Symbol.iterator](), d = f.next(); !d.done; )
                b(d.value[0]), d = f.next();
            }
            u.iterateHeaders = n;
            function s(a, b) {
              for (var f = a.keys(), d = f.next(); !d.done; )
                b(d.value), d = f.next();
            }
            u.iterateHeadersKeys = s;
          }),
          /* 3 */
          /***/
          (function(r, u, i) {
            Object.defineProperty(u, "__esModule", { value: !0 });
            var n = i(2);
            function s(l) {
              if (typeof l != "string" && (l = String(l)), /[^a-z0-9\-#$%&'*+.\^_`|~]/i.test(l))
                throw new TypeError("Invalid character in header field name");
              return l.toLowerCase();
            }
            u.normalizeName = s;
            function a(l) {
              return typeof l != "string" && (l = String(l)), l;
            }
            u.normalizeValue = a;
            function b(l, h) {
              var p = l;
              if (p instanceof Headers && p.getAll)
                return p.getAll(h);
              var v = p.get(h);
              return v && typeof v == "string" ? [v] : v;
            }
            u.getHeaderValues = b;
            function f(l) {
              return l;
            }
            function d(l) {
              var h = l, p = {}, v = [];
              return h.keys ? n.iterateHeadersKeys(h, function(o) {
                p[o] || (p[o] = !0, v.push(o));
              }) : h.forEach ? h.forEach(function(o, c) {
                p[c] || (p[c] = !0, v.push(c));
              }) : n.iterateHeaders(h, function(o) {
                var c = o[0];
                p[c] || (p[c] = !0, v.push(c));
              }), v;
            }
            u.getHeaderKeys = d;
            function y(l) {
              var h = [], p = l.split(", ");
              return p.forEach(function(v) {
                v.split(",").forEach(function(o) {
                  h.push(o);
                });
              }), h;
            }
            u.splitHeaderValue = y;
          })
          /******/
        ])
      );
    });
  })(ee)), ee.exports;
}
var Xt = zt();
class Lt {
  rpc;
  constructor(t) {
    this.rpc = t, this.EchoSwerveSample = this.EchoSwerveSample.bind(this);
  }
  EchoSwerveSample(t, r) {
    return this.rpc.unary(dt, te.fromPartial(t), r);
  }
}
const lt = { serviceName: "service.ChoreoService" }, dt = {
  methodName: "EchoSwerveSample",
  service: lt,
  requestStream: !1,
  responseStream: !1,
  requestType: {
    serializeBinary() {
      return te.encode(this).finish();
    }
  },
  responseType: {
    deserializeBinary(e) {
      const t = ue.decode(e);
      return {
        ...t,
        toObject() {
          return t;
        }
      };
    }
  }
};
class Gt {
  host;
  options;
  constructor(t, r) {
    this.host = t, this.options = r;
  }
  unary(t, r, u) {
    const i = { ...r, ...t.requestType }, n = u && this.options.metadata ? new Xt.BrowserHeaders({ ...this.options?.metadata.headersMap, ...u?.headersMap }) : u ?? this.options.metadata;
    return new Promise((s, a) => {
      Ke.grpc.unary(t, {
        request: i,
        host: this.host,
        metadata: n ?? {},
        ...this.options.transport !== void 0 ? { transport: this.options.transport } : {},
        debug: this.options.debug ?? !1,
        onEnd: function(b) {
          if (b.status === Ke.grpc.Code.OK)
            s(b.message.toObject());
          else {
            const f = new ct(b.statusMessage, b.status, b.trailers);
            a(f);
          }
        }
      });
    });
  }
}
class ct extends globalThis.Error {
  constructor(t, r, u) {
    super(t), this.code = r, this.metadata = u;
  }
}
const Wt = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  ChoreoServiceClientImpl: Lt,
  ChoreoServiceDesc: lt,
  ChoreoServiceEchoSwerveSampleDesc: dt,
  GrpcWebError: ct,
  GrpcWebImpl: Gt,
  commands: Vt
}, Symbol.toStringTag, { value: "Module" }));
export {
  qt as entity,
  Wt as service
};
//# sourceMappingURL=index.mjs.map
