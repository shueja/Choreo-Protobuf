function We() {
  let t = 0, e = 0;
  for (let o = 0; o < 28; o += 7) {
    let i = this.buf[this.pos++];
    if (t |= (i & 127) << o, (i & 128) == 0)
      return this.assertBounds(), [t, e];
  }
  let r = this.buf[this.pos++];
  if (t |= (r & 15) << 28, e = (r & 112) >> 4, (r & 128) == 0)
    return this.assertBounds(), [t, e];
  for (let o = 3; o <= 31; o += 7) {
    let i = this.buf[this.pos++];
    if (e |= (i & 127) << o, (i & 128) == 0)
      return this.assertBounds(), [t, e];
  }
  throw new Error("invalid varint");
}
function gt(t, e, r) {
  for (let n = 0; n < 28; n = n + 7) {
    const f = t >>> n, s = !(!(f >>> 7) && e == 0), m = (s ? f | 128 : f) & 255;
    if (r.push(m), !s)
      return;
  }
  const o = t >>> 28 & 15 | (e & 7) << 4, i = e >> 3 != 0;
  if (r.push((i ? o | 128 : o) & 255), !!i) {
    for (let n = 3; n < 31; n = n + 7) {
      const f = e >>> n, s = !!(f >>> 7), m = (s ? f | 128 : f) & 255;
      if (r.push(m), !s)
        return;
    }
    r.push(e >>> 31 & 1);
  }
}
const lt = 4294967296;
function At(t) {
  const e = t[0] === "-";
  e && (t = t.slice(1));
  const r = 1e6;
  let o = 0, i = 0;
  function n(f, s) {
    const m = Number(t.slice(f, s));
    i *= r, o = o * r + m, o >= lt && (i = i + (o / lt | 0), o = o % lt);
  }
  return n(-24, -18), n(-18, -12), n(-12, -6), n(-6), e ? Pe(o, i) : Tt(o, i);
}
function Xe(t, e) {
  let r = Tt(t, e);
  const o = r.hi & 2147483648;
  o && (r = Pe(r.lo, r.hi));
  const i = _e(r.lo, r.hi);
  return o ? "-" + i : i;
}
function _e(t, e) {
  if ({ lo: t, hi: e } = qe(t, e), e <= 2097151)
    return String(lt * e + t);
  const r = t & 16777215, o = (t >>> 24 | e << 8) & 16777215, i = e >> 16 & 65535;
  let n = r + o * 6777216 + i * 6710656, f = o + i * 8147497, s = i * 2;
  const m = 1e7;
  return n >= m && (f += Math.floor(n / m), n %= m), f >= m && (s += Math.floor(f / m), f %= m), s.toString() + Rt(f) + Rt(n);
}
function qe(t, e) {
  return { lo: t >>> 0, hi: e >>> 0 };
}
function Tt(t, e) {
  return { lo: t | 0, hi: e | 0 };
}
function Pe(t, e) {
  return e = ~e, t ? t = ~t + 1 : e += 1, Tt(t, e);
}
const Rt = (t) => {
  const e = String(t);
  return "0000000".slice(e.length) + e;
};
function It(t, e) {
  if (t >= 0) {
    for (; t > 127; )
      e.push(t & 127 | 128), t = t >>> 7;
    e.push(t);
  } else {
    for (let r = 0; r < 9; r++)
      e.push(t & 127 | 128), t = t >> 7;
    e.push(1);
  }
}
function Ke() {
  let t = this.buf[this.pos++], e = t & 127;
  if ((t & 128) == 0)
    return this.assertBounds(), e;
  if (t = this.buf[this.pos++], e |= (t & 127) << 7, (t & 128) == 0)
    return this.assertBounds(), e;
  if (t = this.buf[this.pos++], e |= (t & 127) << 14, (t & 128) == 0)
    return this.assertBounds(), e;
  if (t = this.buf[this.pos++], e |= (t & 127) << 21, (t & 128) == 0)
    return this.assertBounds(), e;
  t = this.buf[this.pos++], e |= (t & 15) << 28;
  for (let r = 5; (t & 128) !== 0 && r < 10; r++)
    t = this.buf[this.pos++];
  if ((t & 128) != 0)
    throw new Error("invalid varint");
  return this.assertBounds(), e >>> 0;
}
const D = /* @__PURE__ */ Ye();
function Ye() {
  const t = new DataView(new ArrayBuffer(8));
  if (typeof BigInt == "function" && typeof t.getBigInt64 == "function" && typeof t.getBigUint64 == "function" && typeof t.setBigInt64 == "function" && typeof t.setBigUint64 == "function" && (!!globalThis.Deno || typeof process != "object" || typeof process.env != "object" || process.env.BUF_BIGINT_DISABLE !== "1")) {
    const r = BigInt("-9223372036854775808"), o = BigInt("9223372036854775807"), i = BigInt("0"), n = BigInt("18446744073709551615");
    return {
      zero: BigInt(0),
      supported: !0,
      parse(f) {
        const s = typeof f == "bigint" ? f : BigInt(f);
        if (s > o || s < r)
          throw new Error(`invalid int64: ${f}`);
        return s;
      },
      uParse(f) {
        const s = typeof f == "bigint" ? f : BigInt(f);
        if (s > n || s < i)
          throw new Error(`invalid uint64: ${f}`);
        return s;
      },
      enc(f) {
        return t.setBigInt64(0, this.parse(f), !0), {
          lo: t.getInt32(0, !0),
          hi: t.getInt32(4, !0)
        };
      },
      uEnc(f) {
        return t.setBigInt64(0, this.uParse(f), !0), {
          lo: t.getInt32(0, !0),
          hi: t.getInt32(4, !0)
        };
      },
      dec(f, s) {
        return t.setInt32(0, f, !0), t.setInt32(4, s, !0), t.getBigInt64(0, !0);
      },
      uDec(f, s) {
        return t.setInt32(0, f, !0), t.setInt32(4, s, !0), t.getBigUint64(0, !0);
      }
    };
  }
  return {
    zero: "0",
    supported: !1,
    parse(r) {
      return typeof r != "string" && (r = r.toString()), Ht(r), r;
    },
    uParse(r) {
      return typeof r != "string" && (r = r.toString()), Bt(r), r;
    },
    enc(r) {
      return typeof r != "string" && (r = r.toString()), Ht(r), At(r);
    },
    uEnc(r) {
      return typeof r != "string" && (r = r.toString()), Bt(r), At(r);
    },
    dec(r, o) {
      return Xe(r, o);
    },
    uDec(r, o) {
      return _e(r, o);
    }
  };
}
function Ht(t) {
  if (!/^-?[0-9]+$/.test(t))
    throw new Error("invalid int64: " + t);
}
function Bt(t) {
  if (!/^[0-9]+$/.test(t))
    throw new Error("invalid uint64: " + t);
}
const St = /* @__PURE__ */ Symbol.for("@bufbuild/protobuf/text-encoding");
function Je() {
  if (globalThis[St] == null) {
    const t = new globalThis.TextEncoder(), e = new globalThis.TextDecoder();
    globalThis[St] = {
      encodeUtf8(r) {
        return t.encode(r);
      },
      decodeUtf8(r) {
        return e.decode(r);
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
  return globalThis[St];
}
var C;
(function(t) {
  t[t.Varint = 0] = "Varint", t[t.Bit64 = 1] = "Bit64", t[t.LengthDelimited = 2] = "LengthDelimited", t[t.StartGroup = 3] = "StartGroup", t[t.EndGroup = 4] = "EndGroup", t[t.Bit32 = 5] = "Bit32";
})(C || (C = {}));
const Ze = 34028234663852886e22, Qe = -34028234663852886e22, je = 4294967295, tr = 2147483647, er = -2147483648;
class S {
  constructor(e = Je().encodeUtf8) {
    this.encodeUtf8 = e, this.stack = [], this.chunks = [], this.buf = [];
  }
  /**
   * Return all bytes written and reset this writer.
   */
  finish() {
    this.buf.length && (this.chunks.push(new Uint8Array(this.buf)), this.buf = []);
    let e = 0;
    for (let i = 0; i < this.chunks.length; i++)
      e += this.chunks[i].length;
    let r = new Uint8Array(e), o = 0;
    for (let i = 0; i < this.chunks.length; i++)
      r.set(this.chunks[i], o), o += this.chunks[i].length;
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
    let e = this.finish(), r = this.stack.pop();
    if (!r)
      throw new Error("invalid state, fork stack empty");
    return this.chunks = r.chunks, this.buf = r.buf, this.uint32(e.byteLength), this.raw(e);
  }
  /**
   * Writes a tag (field number and wire type).
   *
   * Equivalent to `uint32( (fieldNo << 3 | type) >>> 0 )`.
   *
   * Generated code should compute the tag ahead of time and call `uint32()`.
   */
  tag(e, r) {
    return this.uint32((e << 3 | r) >>> 0);
  }
  /**
   * Write a chunk of raw bytes.
   */
  raw(e) {
    return this.buf.length && (this.chunks.push(new Uint8Array(this.buf)), this.buf = []), this.chunks.push(e), this;
  }
  /**
   * Write a `uint32` value, an unsigned 32 bit varint.
   */
  uint32(e) {
    for (Dt(e); e > 127; )
      this.buf.push(e & 127 | 128), e = e >>> 7;
    return this.buf.push(e), this;
  }
  /**
   * Write a `int32` value, a signed 32 bit varint.
   */
  int32(e) {
    return Nt(e), It(e, this.buf), this;
  }
  /**
   * Write a `bool` value, a variant.
   */
  bool(e) {
    return this.buf.push(e ? 1 : 0), this;
  }
  /**
   * Write a `bytes` value, length-delimited arbitrary data.
   */
  bytes(e) {
    return this.uint32(e.byteLength), this.raw(e);
  }
  /**
   * Write a `string` value, length-delimited data converted to UTF-8 text.
   */
  string(e) {
    let r = this.encodeUtf8(e);
    return this.uint32(r.byteLength), this.raw(r);
  }
  /**
   * Write a `float` value, 32-bit floating point number.
   */
  float(e) {
    rr(e);
    let r = new Uint8Array(4);
    return new DataView(r.buffer).setFloat32(0, e, !0), this.raw(r);
  }
  /**
   * Write a `double` value, a 64-bit floating point number.
   */
  double(e) {
    let r = new Uint8Array(8);
    return new DataView(r.buffer).setFloat64(0, e, !0), this.raw(r);
  }
  /**
   * Write a `fixed32` value, an unsigned, fixed-length 32-bit integer.
   */
  fixed32(e) {
    Dt(e);
    let r = new Uint8Array(4);
    return new DataView(r.buffer).setUint32(0, e, !0), this.raw(r);
  }
  /**
   * Write a `sfixed32` value, a signed, fixed-length 32-bit integer.
   */
  sfixed32(e) {
    Nt(e);
    let r = new Uint8Array(4);
    return new DataView(r.buffer).setInt32(0, e, !0), this.raw(r);
  }
  /**
   * Write a `sint32` value, a signed, zigzag-encoded 32-bit varint.
   */
  sint32(e) {
    return Nt(e), e = (e << 1 ^ e >> 31) >>> 0, It(e, this.buf), this;
  }
  /**
   * Write a `fixed64` value, a signed, fixed-length 64-bit integer.
   */
  sfixed64(e) {
    let r = new Uint8Array(8), o = new DataView(r.buffer), i = D.enc(e);
    return o.setInt32(0, i.lo, !0), o.setInt32(4, i.hi, !0), this.raw(r);
  }
  /**
   * Write a `fixed64` value, an unsigned, fixed-length 64 bit integer.
   */
  fixed64(e) {
    let r = new Uint8Array(8), o = new DataView(r.buffer), i = D.uEnc(e);
    return o.setInt32(0, i.lo, !0), o.setInt32(4, i.hi, !0), this.raw(r);
  }
  /**
   * Write a `int64` value, a signed 64-bit varint.
   */
  int64(e) {
    let r = D.enc(e);
    return gt(r.lo, r.hi, this.buf), this;
  }
  /**
   * Write a `sint64` value, a signed, zig-zag-encoded 64-bit varint.
   */
  sint64(e) {
    const r = D.enc(e), o = r.hi >> 31, i = r.lo << 1 ^ o, n = (r.hi << 1 | r.lo >>> 31) ^ o;
    return gt(i, n, this.buf), this;
  }
  /**
   * Write a `uint64` value, an unsigned 64-bit varint.
   */
  uint64(e) {
    const r = D.uEnc(e);
    return gt(r.lo, r.hi, this.buf), this;
  }
}
class k {
  constructor(e, r = Je().decodeUtf8) {
    this.decodeUtf8 = r, this.varint64 = We, this.uint32 = Ke, this.buf = e, this.len = e.length, this.pos = 0, this.view = new DataView(e.buffer, e.byteOffset, e.byteLength);
  }
  /**
   * Reads a tag - field number and wire type.
   */
  tag() {
    let e = this.uint32(), r = e >>> 3, o = e & 7;
    if (r <= 0 || o < 0 || o > 5)
      throw new Error("illegal tag: field no " + r + " wire type " + o);
    return [r, o];
  }
  /**
   * Skip one element and return the skipped data.
   *
   * When skipping StartGroup, provide the tags field number to check for
   * matching field number in the EndGroup tag.
   */
  skip(e, r) {
    let o = this.pos;
    switch (e) {
      case C.Varint:
        for (; this.buf[this.pos++] & 128; )
          ;
        break;
      // @ts-ignore TS7029: Fallthrough case in switch -- ignore instead of expect-error for compiler settings without noFallthroughCasesInSwitch: true
      case C.Bit64:
        this.pos += 4;
      case C.Bit32:
        this.pos += 4;
        break;
      case C.LengthDelimited:
        let i = this.uint32();
        this.pos += i;
        break;
      case C.StartGroup:
        for (; ; ) {
          const [n, f] = this.tag();
          if (f === C.EndGroup) {
            if (r !== void 0 && n !== r)
              throw new Error("invalid end group tag");
            break;
          }
          this.skip(f, n);
        }
        break;
      default:
        throw new Error("cant skip wire type " + e);
    }
    return this.assertBounds(), this.buf.subarray(o, this.pos);
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
    let e = this.uint32();
    return e >>> 1 ^ -(e & 1);
  }
  /**
   * Read a `int64` field, a signed 64-bit varint.
   */
  int64() {
    return D.dec(...this.varint64());
  }
  /**
   * Read a `uint64` field, an unsigned 64-bit varint.
   */
  uint64() {
    return D.uDec(...this.varint64());
  }
  /**
   * Read a `sint64` field, a signed, zig-zag-encoded 64-bit varint.
   */
  sint64() {
    let [e, r] = this.varint64(), o = -(e & 1);
    return e = (e >>> 1 | (r & 1) << 31) ^ o, r = r >>> 1 ^ o, D.dec(e, r);
  }
  /**
   * Read a `bool` field, a variant.
   */
  bool() {
    let [e, r] = this.varint64();
    return e !== 0 || r !== 0;
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
    return D.uDec(this.sfixed32(), this.sfixed32());
  }
  /**
   * Read a `fixed64` field, a signed, fixed-length 64-bit integer.
   */
  sfixed64() {
    return D.dec(this.sfixed32(), this.sfixed32());
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
    let e = this.uint32(), r = this.pos;
    return this.pos += e, this.assertBounds(), this.buf.subarray(r, r + e);
  }
  /**
   * Read a `string` field, length-delimited data converted to UTF-8 text.
   */
  string() {
    return this.decodeUtf8(this.bytes());
  }
}
function Nt(t) {
  if (typeof t == "string")
    t = Number(t);
  else if (typeof t != "number")
    throw new Error("invalid int32: " + typeof t);
  if (!Number.isInteger(t) || t > tr || t < er)
    throw new Error("invalid int32: " + t);
}
function Dt(t) {
  if (typeof t == "string")
    t = Number(t);
  else if (typeof t != "number")
    throw new Error("invalid uint32: " + typeof t);
  if (!Number.isInteger(t) || t > je || t < 0)
    throw new Error("invalid uint32: " + t);
}
function rr(t) {
  if (typeof t == "string") {
    const e = t;
    if (t = Number(t), Number.isNaN(t) && e !== "NaN")
      throw new Error("invalid float32: " + e);
  } else if (typeof t != "number")
    throw new Error("invalid float32: " + typeof t);
  if (Number.isFinite(t) && (t > Ze || t < Qe))
    throw new Error("invalid float32: " + t);
}
function Ct() {
  return { t: 0, x: 0, y: 0, heading: 0, vl: 0, vr: 0, omega: 0, al: 0, ar: 0, alpha: 0, fl: 0, fr: 0 };
}
const W = {
  encode(t, e = new S()) {
    return t.t !== 0 && e.uint32(9).double(t.t), t.x !== 0 && e.uint32(17).double(t.x), t.y !== 0 && e.uint32(25).double(t.y), t.heading !== 0 && e.uint32(33).double(t.heading), t.vl !== 0 && e.uint32(41).double(t.vl), t.vr !== 0 && e.uint32(49).double(t.vr), t.omega !== 0 && e.uint32(57).double(t.omega), t.al !== 0 && e.uint32(65).double(t.al), t.ar !== 0 && e.uint32(73).double(t.ar), t.alpha !== 0 && e.uint32(81).double(t.alpha), t.fl !== 0 && e.uint32(89).double(t.fl), t.fr !== 0 && e.uint32(97).double(t.fr), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Ct();
    for (; r.pos < o; ) {
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
  fromJSON(t) {
    return {
      t: I(t.t) ? globalThis.Number(t.t) : 0,
      x: I(t.x) ? globalThis.Number(t.x) : 0,
      y: I(t.y) ? globalThis.Number(t.y) : 0,
      heading: I(t.heading) ? globalThis.Number(t.heading) : 0,
      vl: I(t.vl) ? globalThis.Number(t.vl) : 0,
      vr: I(t.vr) ? globalThis.Number(t.vr) : 0,
      omega: I(t.omega) ? globalThis.Number(t.omega) : 0,
      al: I(t.al) ? globalThis.Number(t.al) : 0,
      ar: I(t.ar) ? globalThis.Number(t.ar) : 0,
      alpha: I(t.alpha) ? globalThis.Number(t.alpha) : 0,
      fl: I(t.fl) ? globalThis.Number(t.fl) : 0,
      fr: I(t.fr) ? globalThis.Number(t.fr) : 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.t !== 0 && (e.t = t.t), t.x !== 0 && (e.x = t.x), t.y !== 0 && (e.y = t.y), t.heading !== 0 && (e.heading = t.heading), t.vl !== 0 && (e.vl = t.vl), t.vr !== 0 && (e.vr = t.vr), t.omega !== 0 && (e.omega = t.omega), t.al !== 0 && (e.al = t.al), t.ar !== 0 && (e.ar = t.ar), t.alpha !== 0 && (e.alpha = t.alpha), t.fl !== 0 && (e.fl = t.fl), t.fr !== 0 && (e.fr = t.fr), e;
  },
  create(t) {
    return W.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Ct();
    return e.t = t.t ?? 0, e.x = t.x ?? 0, e.y = t.y ?? 0, e.heading = t.heading ?? 0, e.vl = t.vl ?? 0, e.vr = t.vr ?? 0, e.omega = t.omega ?? 0, e.al = t.al ?? 0, e.ar = t.ar ?? 0, e.alpha = t.alpha ?? 0, e.fl = t.fl ?? 0, e.fr = t.fr ?? 0, e;
  }
};
function I(t) {
  return t != null;
}
var Me = /* @__PURE__ */ ((t) => (t[t.DRIVETYPE_SWERVE = 0] = "DRIVETYPE_SWERVE", t[t.DRIVETYPE_DIFFERENTIAL = 1] = "DRIVETYPE_DIFFERENTIAL", t[t.DRIVETYPE_MECANUM = 2] = "DRIVETYPE_MECANUM", t[t.UNRECOGNIZED = -1] = "UNRECOGNIZED", t))(Me || {});
function Ot(t) {
  switch (t) {
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
function Ae(t) {
  switch (t) {
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
function Vt() {
  return { value: 0, expr: "" };
}
const y = {
  encode(t, e = new S()) {
    return t.value !== 0 && e.uint32(9).double(t.value), t.expr !== "" && e.uint32(18).string(t.expr), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Vt();
    for (; r.pos < o; ) {
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
  fromJSON(t) {
    return {
      value: Lt(t.value) ? globalThis.Number(t.value) : 0,
      expr: Lt(t.expr) ? globalThis.String(t.expr) : ""
    };
  },
  toJSON(t) {
    const e = {};
    return t.value !== 0 && (e.value = t.value), t.expr !== "" && (e.expr = t.expr), e;
  },
  create(t) {
    return y.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Vt();
    return e.value = t.value ?? 0, e.expr = t.expr ?? "", e;
  }
};
function Lt(t) {
  return t != null;
}
function Ft() {
  return { max: 0 };
}
const V = {
  encode(t, e = new S()) {
    return t.max !== 0 && e.uint32(9).double(t.max), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Ft();
    for (; r.pos < o; ) {
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
  fromJSON(t) {
    return { max: Ie(t.max) ? globalThis.Number(t.max) : 0 };
  },
  toJSON(t) {
    const e = {};
    return t.max !== 0 && (e.max = t.max), e;
  },
  create(t) {
    return V.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Ft();
    return e.max = t.max ?? 0, e;
  }
};
function Ut() {
  return { test: "" };
}
const Re = {
  encode(t, e = new S()) {
    return t.test !== "" && e.uint32(10).string(t.test), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Ut();
    for (; r.pos < o; ) {
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
  fromJSON(t) {
    return { test: Ie(t.test) ? globalThis.String(t.test) : "" };
  },
  toJSON(t) {
    const e = {};
    return t.test !== "" && (e.test = t.test), e;
  },
  create(t) {
    return Re.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Ut();
    return e.test = t.test ?? "", e;
  }
};
function Ie(t) {
  return t != null;
}
function $t() {
  return { max: void 0 };
}
const L = {
  encode(t, e = new S()) {
    return t.max !== void 0 && y.encode(t.max, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = $t();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.max = y.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return { max: Be(t.max) ? y.fromJSON(t.max) : void 0 };
  },
  toJSON(t) {
    const e = {};
    return t.max !== void 0 && (e.max = y.toJSON(t.max)), e;
  },
  create(t) {
    return L.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = $t();
    return e.max = t.max !== void 0 && t.max !== null ? y.fromPartial(t.max) : void 0, e;
  }
};
function zt() {
  return { test: "" };
}
const He = {
  encode(t, e = new S()) {
    return t.test !== "" && e.uint32(10).string(t.test), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = zt();
    for (; r.pos < o; ) {
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
  fromJSON(t) {
    return { test: Be(t.test) ? globalThis.String(t.test) : "" };
  },
  toJSON(t) {
    const e = {};
    return t.test !== "" && (e.test = t.test), e;
  },
  create(t) {
    return He.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = zt();
    return e.test = t.test ?? "", e;
  }
};
function Be(t) {
  return t != null;
}
const nr = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DoubleMaxVelocity: V,
  ExprMaxVelocity: L,
  TestDouble: Re,
  TestExpr: He
}, Symbol.toStringTag, { value: "Module" }));
function Gt() {
  return { max: 0 };
}
const F = {
  encode(t, e = new S()) {
    return t.max !== 0 && e.uint32(9).double(t.max), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Gt();
    for (; r.pos < o; ) {
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
  fromJSON(t) {
    return { max: ir(t.max) ? globalThis.Number(t.max) : 0 };
  },
  toJSON(t) {
    const e = {};
    return t.max !== 0 && (e.max = t.max), e;
  },
  create(t) {
    return F.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Gt();
    return e.max = t.max ?? 0, e;
  }
};
function ir(t) {
  return t != null;
}
function Wt() {
  return { max: void 0 };
}
const U = {
  encode(t, e = new S()) {
    return t.max !== void 0 && y.encode(t.max, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Wt();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.max = y.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return { max: or(t.max) ? y.fromJSON(t.max) : void 0 };
  },
  toJSON(t) {
    const e = {};
    return t.max !== void 0 && (e.max = y.toJSON(t.max)), e;
  },
  create(t) {
    return U.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Wt();
    return e.max = t.max !== void 0 && t.max !== null ? y.fromPartial(t.max) : void 0, e;
  }
};
function or(t) {
  return t != null;
}
const ar = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DoubleMaxAcceleration: F,
  ExprMaxAcceleration: U
}, Symbol.toStringTag, { value: "Module" }));
function Xt() {
  return {};
}
const X = {
  encode(t, e = new S()) {
    return e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Xt();
    for (; r.pos < o; ) {
      const n = r.uint32();
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {};
  },
  toJSON(t) {
    return {};
  },
  create(t) {
    return X.fromPartial(t ?? {});
  },
  fromPartial(t) {
    return Xt();
  }
};
function qt() {
  return {};
}
const q = {
  encode(t, e = new S()) {
    return e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = qt();
    for (; r.pos < o; ) {
      const n = r.uint32();
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {};
  },
  toJSON(t) {
    return {};
  },
  create(t) {
    return q.fromPartial(t ?? {});
  },
  fromPartial(t) {
    return qt();
  }
};
function Kt() {
  return { idx: 0 };
}
const K = {
  encode(t, e = new S()) {
    return t.idx !== 0 && e.uint32(8).uint64(t.idx), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Kt();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 8)
            break;
          i.idx = sr(r.uint64());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return { idx: ct(t.idx) ? globalThis.Number(t.idx) : 0 };
  },
  toJSON(t) {
    const e = {};
    return t.idx !== 0 && (e.idx = Math.round(t.idx)), e;
  },
  create(t) {
    return K.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Kt();
    return e.idx = t.idx ?? 0, e;
  }
};
function Yt() {
  return { id: void 0 };
}
const P = {
  encode(t, e = new S()) {
    switch (t.id?.$case) {
      case "first":
        X.encode(t.id.first, e.uint32(10).fork()).join();
        break;
      case "last":
        q.encode(t.id.last, e.uint32(18).fork()).join();
        break;
      case "idx":
        K.encode(t.id.idx, e.uint32(26).fork()).join();
        break;
    }
    return e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Yt();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.id = { $case: "first", first: X.decode(r, r.uint32()) };
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.id = { $case: "last", last: q.decode(r, r.uint32()) };
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.id = { $case: "idx", idx: K.decode(r, r.uint32()) };
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      id: ct(t.first) ? { $case: "first", first: X.fromJSON(t.first) } : ct(t.last) ? { $case: "last", last: q.fromJSON(t.last) } : ct(t.idx) ? { $case: "idx", idx: K.fromJSON(t.idx) } : void 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.id?.$case === "first" ? e.first = X.toJSON(t.id.first) : t.id?.$case === "last" ? e.last = q.toJSON(t.id.last) : t.id?.$case === "idx" && (e.idx = K.toJSON(t.id.idx)), e;
  },
  create(t) {
    return P.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Yt();
    switch (t.id?.$case) {
      case "first": {
        t.id?.first !== void 0 && t.id?.first !== null && (e.id = { $case: "first", first: X.fromPartial(t.id.first) });
        break;
      }
      case "last": {
        t.id?.last !== void 0 && t.id?.last !== null && (e.id = { $case: "last", last: q.fromPartial(t.id.last) });
        break;
      }
      case "idx": {
        t.id?.idx !== void 0 && t.id?.idx !== null && (e.id = { $case: "idx", idx: K.fromPartial(t.id.idx) });
        break;
      }
    }
    return e;
  }
};
function sr(t) {
  const e = globalThis.Number(t.toString());
  if (e > globalThis.Number.MAX_SAFE_INTEGER)
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  if (e < globalThis.Number.MIN_SAFE_INTEGER)
    throw new globalThis.Error("Value is smaller than Number.MIN_SAFE_INTEGER");
  return e;
}
function ct(t) {
  return t != null;
}
function Zt() {
  return { enabled: !1, from: void 0, to: void 0, data: void 0 };
}
const Y = {
  encode(t, e = new S()) {
    switch (t.enabled !== !1 && e.uint32(8).bool(t.enabled), t.from !== void 0 && P.encode(t.from, e.uint32(18).fork()).join(), t.to !== void 0 && P.encode(t.to, e.uint32(26).fork()).join(), t.data?.$case) {
      case "maxVelocity":
        V.encode(t.data.maxVelocity, e.uint32(34).fork()).join();
        break;
      case "maxAcceleration":
        F.encode(t.data.maxAcceleration, e.uint32(42).fork()).join();
        break;
    }
    return e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Zt();
    for (; r.pos < o; ) {
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
          i.from = P.decode(r, r.uint32());
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.to = P.decode(r, r.uint32());
          continue;
        }
        case 4: {
          if (n !== 34)
            break;
          i.data = { $case: "maxVelocity", maxVelocity: V.decode(r, r.uint32()) };
          continue;
        }
        case 5: {
          if (n !== 42)
            break;
          i.data = {
            $case: "maxAcceleration",
            maxAcceleration: F.decode(r, r.uint32())
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
  fromJSON(t) {
    return {
      enabled: z(t.enabled) ? globalThis.Boolean(t.enabled) : !1,
      from: z(t.from) ? P.fromJSON(t.from) : void 0,
      to: z(t.to) ? P.fromJSON(t.to) : void 0,
      data: z(t.maxVelocity) ? { $case: "maxVelocity", maxVelocity: V.fromJSON(t.maxVelocity) } : z(t.max_velocity) ? { $case: "maxVelocity", maxVelocity: V.fromJSON(t.max_velocity) } : z(t.maxAcceleration) ? { $case: "maxAcceleration", maxAcceleration: F.fromJSON(t.maxAcceleration) } : z(t.max_acceleration) ? { $case: "maxAcceleration", maxAcceleration: F.fromJSON(t.max_acceleration) } : void 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.enabled !== !1 && (e.enabled = t.enabled), t.from !== void 0 && (e.from = P.toJSON(t.from)), t.to !== void 0 && (e.to = P.toJSON(t.to)), t.data?.$case === "maxVelocity" ? e.maxVelocity = V.toJSON(t.data.maxVelocity) : t.data?.$case === "maxAcceleration" && (e.maxAcceleration = F.toJSON(t.data.maxAcceleration)), e;
  },
  create(t) {
    return Y.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Zt();
    switch (e.enabled = t.enabled ?? !1, e.from = t.from !== void 0 && t.from !== null ? P.fromPartial(t.from) : void 0, e.to = t.to !== void 0 && t.to !== null ? P.fromPartial(t.to) : void 0, t.data?.$case) {
      case "maxVelocity": {
        t.data?.maxVelocity !== void 0 && t.data?.maxVelocity !== null && (e.data = { $case: "maxVelocity", maxVelocity: V.fromPartial(t.data.maxVelocity) });
        break;
      }
      case "maxAcceleration": {
        t.data?.maxAcceleration !== void 0 && t.data?.maxAcceleration !== null && (e.data = {
          $case: "maxAcceleration",
          maxAcceleration: F.fromPartial(t.data.maxAcceleration)
        });
        break;
      }
    }
    return e;
  }
};
function z(t) {
  return t != null;
}
function Qt() {
  return { enabled: !1, from: void 0, to: void 0, data: void 0 };
}
const Z = {
  encode(t, e = new S()) {
    switch (t.enabled !== !1 && e.uint32(8).bool(t.enabled), t.from !== void 0 && P.encode(t.from, e.uint32(18).fork()).join(), t.to !== void 0 && P.encode(t.to, e.uint32(26).fork()).join(), t.data?.$case) {
      case "maxVelocity":
        L.encode(t.data.maxVelocity, e.uint32(34).fork()).join();
        break;
      case "maxAcceleration":
        U.encode(t.data.maxAcceleration, e.uint32(42).fork()).join();
        break;
    }
    return e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Qt();
    for (; r.pos < o; ) {
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
          i.from = P.decode(r, r.uint32());
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.to = P.decode(r, r.uint32());
          continue;
        }
        case 4: {
          if (n !== 34)
            break;
          i.data = { $case: "maxVelocity", maxVelocity: L.decode(r, r.uint32()) };
          continue;
        }
        case 5: {
          if (n !== 42)
            break;
          i.data = {
            $case: "maxAcceleration",
            maxAcceleration: U.decode(r, r.uint32())
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
  fromJSON(t) {
    return {
      enabled: G(t.enabled) ? globalThis.Boolean(t.enabled) : !1,
      from: G(t.from) ? P.fromJSON(t.from) : void 0,
      to: G(t.to) ? P.fromJSON(t.to) : void 0,
      data: G(t.maxVelocity) ? { $case: "maxVelocity", maxVelocity: L.fromJSON(t.maxVelocity) } : G(t.max_velocity) ? { $case: "maxVelocity", maxVelocity: L.fromJSON(t.max_velocity) } : G(t.maxAcceleration) ? { $case: "maxAcceleration", maxAcceleration: U.fromJSON(t.maxAcceleration) } : G(t.max_acceleration) ? { $case: "maxAcceleration", maxAcceleration: U.fromJSON(t.max_acceleration) } : void 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.enabled !== !1 && (e.enabled = t.enabled), t.from !== void 0 && (e.from = P.toJSON(t.from)), t.to !== void 0 && (e.to = P.toJSON(t.to)), t.data?.$case === "maxVelocity" ? e.maxVelocity = L.toJSON(t.data.maxVelocity) : t.data?.$case === "maxAcceleration" && (e.maxAcceleration = U.toJSON(t.data.maxAcceleration)), e;
  },
  create(t) {
    return Z.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Qt();
    switch (e.enabled = t.enabled ?? !1, e.from = t.from !== void 0 && t.from !== null ? P.fromPartial(t.from) : void 0, e.to = t.to !== void 0 && t.to !== null ? P.fromPartial(t.to) : void 0, t.data?.$case) {
      case "maxVelocity": {
        t.data?.maxVelocity !== void 0 && t.data?.maxVelocity !== null && (e.data = { $case: "maxVelocity", maxVelocity: L.fromPartial(t.data.maxVelocity) });
        break;
      }
      case "maxAcceleration": {
        t.data?.maxAcceleration !== void 0 && t.data?.maxAcceleration !== null && (e.data = {
          $case: "maxAcceleration",
          maxAcceleration: U.fromPartial(t.data.maxAcceleration)
        });
        break;
      }
    }
    return e;
  }
};
function G(t) {
  return t != null;
}
const fr = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DoubleConstraint: Y,
  ExprConstraint: Z,
  max_acceleration: ar,
  maxvelocity: nr
}, Symbol.toStringTag, { value: "Module" }));
function jt() {
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
const Q = {
  encode(t, e = new S()) {
    return t.x !== 0 && e.uint32(9).double(t.x), t.y !== 0 && e.uint32(17).double(t.y), t.heading !== 0 && e.uint32(25).double(t.heading), t.intervals !== 0 && e.uint32(32).uint64(t.intervals), t.split !== !1 && e.uint32(40).bool(t.split), t.fixTranslation !== !1 && e.uint32(48).bool(t.fixTranslation), t.fixHeading !== !1 && e.uint32(56).bool(t.fixHeading), t.overrideIntervals !== !1 && e.uint32(64).bool(t.overrideIntervals), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = jt();
    for (; r.pos < o; ) {
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
          i.intervals = ur(r.uint64());
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
  fromJSON(t) {
    return {
      x: H(t.x) ? globalThis.Number(t.x) : 0,
      y: H(t.y) ? globalThis.Number(t.y) : 0,
      heading: H(t.heading) ? globalThis.Number(t.heading) : 0,
      intervals: H(t.intervals) ? globalThis.Number(t.intervals) : 0,
      split: H(t.split) ? globalThis.Boolean(t.split) : !1,
      fixTranslation: H(t.fixTranslation) ? globalThis.Boolean(t.fixTranslation) : H(t.fix_translation) ? globalThis.Boolean(t.fix_translation) : !1,
      fixHeading: H(t.fixHeading) ? globalThis.Boolean(t.fixHeading) : H(t.fix_heading) ? globalThis.Boolean(t.fix_heading) : !1,
      overrideIntervals: H(t.overrideIntervals) ? globalThis.Boolean(t.overrideIntervals) : H(t.override_intervals) ? globalThis.Boolean(t.override_intervals) : !1
    };
  },
  toJSON(t) {
    const e = {};
    return t.x !== 0 && (e.x = t.x), t.y !== 0 && (e.y = t.y), t.heading !== 0 && (e.heading = t.heading), t.intervals !== 0 && (e.intervals = Math.round(t.intervals)), t.split !== !1 && (e.split = t.split), t.fixTranslation !== !1 && (e.fixTranslation = t.fixTranslation), t.fixHeading !== !1 && (e.fixHeading = t.fixHeading), t.overrideIntervals !== !1 && (e.overrideIntervals = t.overrideIntervals), e;
  },
  create(t) {
    return Q.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = jt();
    return e.x = t.x ?? 0, e.y = t.y ?? 0, e.heading = t.heading ?? 0, e.intervals = t.intervals ?? 0, e.split = t.split ?? !1, e.fixTranslation = t.fixTranslation ?? !1, e.fixHeading = t.fixHeading ?? !1, e.overrideIntervals = t.overrideIntervals ?? !1, e;
  }
};
function ur(t) {
  const e = globalThis.Number(t.toString());
  if (e > globalThis.Number.MAX_SAFE_INTEGER)
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  if (e < globalThis.Number.MIN_SAFE_INTEGER)
    throw new globalThis.Error("Value is smaller than Number.MIN_SAFE_INTEGER");
  return e;
}
function H(t) {
  return t != null;
}
function te() {
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
const j = {
  encode(t, e = new S()) {
    return t.x !== void 0 && y.encode(t.x, e.uint32(10).fork()).join(), t.y !== void 0 && y.encode(t.y, e.uint32(18).fork()).join(), t.heading !== void 0 && y.encode(t.heading, e.uint32(26).fork()).join(), t.intervals !== 0 && e.uint32(32).uint64(t.intervals), t.split !== !1 && e.uint32(40).bool(t.split), t.fixTranslation !== !1 && e.uint32(48).bool(t.fixTranslation), t.fixHeading !== !1 && e.uint32(56).bool(t.fixHeading), t.overrideIntervals !== !1 && e.uint32(64).bool(t.overrideIntervals), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = te();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.x = y.decode(r, r.uint32());
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.y = y.decode(r, r.uint32());
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.heading = y.decode(r, r.uint32());
          continue;
        }
        case 4: {
          if (n !== 32)
            break;
          i.intervals = dr(r.uint64());
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
  fromJSON(t) {
    return {
      x: B(t.x) ? y.fromJSON(t.x) : void 0,
      y: B(t.y) ? y.fromJSON(t.y) : void 0,
      heading: B(t.heading) ? y.fromJSON(t.heading) : void 0,
      intervals: B(t.intervals) ? globalThis.Number(t.intervals) : 0,
      split: B(t.split) ? globalThis.Boolean(t.split) : !1,
      fixTranslation: B(t.fixTranslation) ? globalThis.Boolean(t.fixTranslation) : B(t.fix_translation) ? globalThis.Boolean(t.fix_translation) : !1,
      fixHeading: B(t.fixHeading) ? globalThis.Boolean(t.fixHeading) : B(t.fix_heading) ? globalThis.Boolean(t.fix_heading) : !1,
      overrideIntervals: B(t.overrideIntervals) ? globalThis.Boolean(t.overrideIntervals) : B(t.override_intervals) ? globalThis.Boolean(t.override_intervals) : !1
    };
  },
  toJSON(t) {
    const e = {};
    return t.x !== void 0 && (e.x = y.toJSON(t.x)), t.y !== void 0 && (e.y = y.toJSON(t.y)), t.heading !== void 0 && (e.heading = y.toJSON(t.heading)), t.intervals !== 0 && (e.intervals = Math.round(t.intervals)), t.split !== !1 && (e.split = t.split), t.fixTranslation !== !1 && (e.fixTranslation = t.fixTranslation), t.fixHeading !== !1 && (e.fixHeading = t.fixHeading), t.overrideIntervals !== !1 && (e.overrideIntervals = t.overrideIntervals), e;
  },
  create(t) {
    return j.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = te();
    return e.x = t.x !== void 0 && t.x !== null ? y.fromPartial(t.x) : void 0, e.y = t.y !== void 0 && t.y !== null ? y.fromPartial(t.y) : void 0, e.heading = t.heading !== void 0 && t.heading !== null ? y.fromPartial(t.heading) : void 0, e.intervals = t.intervals ?? 0, e.split = t.split ?? !1, e.fixTranslation = t.fixTranslation ?? !1, e.fixHeading = t.fixHeading ?? !1, e.overrideIntervals = t.overrideIntervals ?? !1, e;
  }
};
function dr(t) {
  const e = globalThis.Number(t.toString());
  if (e > globalThis.Number.MAX_SAFE_INTEGER)
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  if (e < globalThis.Number.MIN_SAFE_INTEGER)
    throw new globalThis.Error("Value is smaller than Number.MIN_SAFE_INTEGER");
  return e;
}
function B(t) {
  return t != null;
}
const lr = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DoubleWaypoint: Q,
  ExprWaypoint: j
}, Symbol.toStringTag, { value: "Module" }));
function ee() {
  return { targetDt: 0, waypoints: [], constraints: [] };
}
const tt = {
  encode(t, e = new S()) {
    t.targetDt !== 0 && e.uint32(9).double(t.targetDt);
    for (const r of t.waypoints)
      Q.encode(r, e.uint32(18).fork()).join();
    for (const r of t.constraints)
      Y.encode(r, e.uint32(26).fork()).join();
    return e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = ee();
    for (; r.pos < o; ) {
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
          i.waypoints.push(Q.decode(r, r.uint32()));
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.constraints.push(Y.decode(r, r.uint32()));
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      targetDt: re(t.targetDt) ? globalThis.Number(t.targetDt) : re(t.target_dt) ? globalThis.Number(t.target_dt) : 0,
      waypoints: globalThis.Array.isArray(t?.waypoints) ? t.waypoints.map((e) => Q.fromJSON(e)) : [],
      constraints: globalThis.Array.isArray(t?.constraints) ? t.constraints.map((e) => Y.fromJSON(e)) : []
    };
  },
  toJSON(t) {
    const e = {};
    return t.targetDt !== 0 && (e.targetDt = t.targetDt), t.waypoints?.length && (e.waypoints = t.waypoints.map((r) => Q.toJSON(r))), t.constraints?.length && (e.constraints = t.constraints.map((r) => Y.toJSON(r))), e;
  },
  create(t) {
    return tt.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = ee();
    return e.targetDt = t.targetDt ?? 0, e.waypoints = t.waypoints?.map((r) => Q.fromPartial(r)) || [], e.constraints = t.constraints?.map((r) => Y.fromPartial(r)) || [], e;
  }
};
function re(t) {
  return t != null;
}
function ne() {
  return { targetDt: void 0, waypoints: [], constraints: [] };
}
const et = {
  encode(t, e = new S()) {
    t.targetDt !== void 0 && y.encode(t.targetDt, e.uint32(10).fork()).join();
    for (const r of t.waypoints)
      j.encode(r, e.uint32(18).fork()).join();
    for (const r of t.constraints)
      Z.encode(r, e.uint32(26).fork()).join();
    return e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = ne();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.targetDt = y.decode(r, r.uint32());
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.waypoints.push(j.decode(r, r.uint32()));
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.constraints.push(Z.decode(r, r.uint32()));
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      targetDt: ie(t.targetDt) ? y.fromJSON(t.targetDt) : ie(t.target_dt) ? y.fromJSON(t.target_dt) : void 0,
      waypoints: globalThis.Array.isArray(t?.waypoints) ? t.waypoints.map((e) => j.fromJSON(e)) : [],
      constraints: globalThis.Array.isArray(t?.constraints) ? t.constraints.map((e) => Z.fromJSON(e)) : []
    };
  },
  toJSON(t) {
    const e = {};
    return t.targetDt !== void 0 && (e.targetDt = y.toJSON(t.targetDt)), t.waypoints?.length && (e.waypoints = t.waypoints.map((r) => j.toJSON(r))), t.constraints?.length && (e.constraints = t.constraints.map((r) => Z.toJSON(r))), e;
  },
  create(t) {
    return et.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = ne();
    return e.targetDt = t.targetDt !== void 0 && t.targetDt !== null ? y.fromPartial(t.targetDt) : void 0, e.waypoints = t.waypoints?.map((r) => j.fromPartial(r)) || [], e.constraints = t.constraints?.map((r) => Z.fromPartial(r)) || [], e;
  }
};
function ie(t) {
  return t != null;
}
function oe() {
  return { x: 0, y: 0 };
}
const w = {
  encode(t, e = new S()) {
    return t.x !== 0 && e.uint32(9).double(t.x), t.y !== 0 && e.uint32(17).double(t.y), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = oe();
    for (; r.pos < o; ) {
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
  fromJSON(t) {
    return {
      x: T(t.x) ? globalThis.Number(t.x) : 0,
      y: T(t.y) ? globalThis.Number(t.y) : 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.x !== 0 && (e.x = t.x), t.y !== 0 && (e.y = t.y), e;
  },
  create(t) {
    return w.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = oe();
    return e.x = t.x ?? 0, e.y = t.y ?? 0, e;
  }
};
function ae() {
  return { front: 0, left: 0, right: 0, back: 0 };
}
const rt = {
  encode(t, e = new S()) {
    return t.front !== 0 && e.uint32(9).double(t.front), t.left !== 0 && e.uint32(17).double(t.left), t.right !== 0 && e.uint32(25).double(t.right), t.back !== 0 && e.uint32(33).double(t.back), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = ae();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 9)
            break;
          i.front = r.double();
          continue;
        }
        case 2: {
          if (n !== 17)
            break;
          i.left = r.double();
          continue;
        }
        case 3: {
          if (n !== 25)
            break;
          i.right = r.double();
          continue;
        }
        case 4: {
          if (n !== 33)
            break;
          i.back = r.double();
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      front: T(t.front) ? globalThis.Number(t.front) : 0,
      left: T(t.left) ? globalThis.Number(t.left) : 0,
      right: T(t.right) ? globalThis.Number(t.right) : 0,
      back: T(t.back) ? globalThis.Number(t.back) : 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.front !== 0 && (e.front = t.front), t.left !== 0 && (e.left = t.left), t.right !== 0 && (e.right = t.right), t.back !== 0 && (e.back = t.back), e;
  },
  create(t) {
    return rt.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = ae();
    return e.front = t.front ?? 0, e.left = t.left ?? 0, e.right = t.right ?? 0, e.back = t.back ?? 0, e;
  }
};
function se() {
  return {
    mass: 0,
    inertia: 0,
    gearing: 0,
    radius: 0,
    vmax: 0,
    tmax: 0,
    cof: 0,
    differentialTrackWidth: 0,
    bumper: void 0,
    frontLeft: void 0,
    frontRight: void 0,
    backLeft: void 0,
    backRight: void 0
  };
}
const nt = {
  encode(t, e = new S()) {
    return t.mass !== 0 && e.uint32(9).double(t.mass), t.inertia !== 0 && e.uint32(17).double(t.inertia), t.gearing !== 0 && e.uint32(25).double(t.gearing), t.radius !== 0 && e.uint32(33).double(t.radius), t.vmax !== 0 && e.uint32(41).double(t.vmax), t.tmax !== 0 && e.uint32(49).double(t.tmax), t.cof !== 0 && e.uint32(57).double(t.cof), t.differentialTrackWidth !== 0 && e.uint32(65).double(t.differentialTrackWidth), t.bumper !== void 0 && rt.encode(t.bumper, e.uint32(74).fork()).join(), t.frontLeft !== void 0 && w.encode(t.frontLeft, e.uint32(82).fork()).join(), t.frontRight !== void 0 && w.encode(t.frontRight, e.uint32(90).fork()).join(), t.backLeft !== void 0 && w.encode(t.backLeft, e.uint32(98).fork()).join(), t.backRight !== void 0 && w.encode(t.backRight, e.uint32(106).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = se();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 9)
            break;
          i.mass = r.double();
          continue;
        }
        case 2: {
          if (n !== 17)
            break;
          i.inertia = r.double();
          continue;
        }
        case 3: {
          if (n !== 25)
            break;
          i.gearing = r.double();
          continue;
        }
        case 4: {
          if (n !== 33)
            break;
          i.radius = r.double();
          continue;
        }
        case 5: {
          if (n !== 41)
            break;
          i.vmax = r.double();
          continue;
        }
        case 6: {
          if (n !== 49)
            break;
          i.tmax = r.double();
          continue;
        }
        case 7: {
          if (n !== 57)
            break;
          i.cof = r.double();
          continue;
        }
        case 8: {
          if (n !== 65)
            break;
          i.differentialTrackWidth = r.double();
          continue;
        }
        case 9: {
          if (n !== 74)
            break;
          i.bumper = rt.decode(r, r.uint32());
          continue;
        }
        case 10: {
          if (n !== 82)
            break;
          i.frontLeft = w.decode(r, r.uint32());
          continue;
        }
        case 11: {
          if (n !== 90)
            break;
          i.frontRight = w.decode(r, r.uint32());
          continue;
        }
        case 12: {
          if (n !== 98)
            break;
          i.backLeft = w.decode(r, r.uint32());
          continue;
        }
        case 13: {
          if (n !== 106)
            break;
          i.backRight = w.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      mass: T(t.mass) ? globalThis.Number(t.mass) : 0,
      inertia: T(t.inertia) ? globalThis.Number(t.inertia) : 0,
      gearing: T(t.gearing) ? globalThis.Number(t.gearing) : 0,
      radius: T(t.radius) ? globalThis.Number(t.radius) : 0,
      vmax: T(t.vmax) ? globalThis.Number(t.vmax) : 0,
      tmax: T(t.tmax) ? globalThis.Number(t.tmax) : 0,
      cof: T(t.cof) ? globalThis.Number(t.cof) : 0,
      differentialTrackWidth: T(t.differentialTrackWidth) ? globalThis.Number(t.differentialTrackWidth) : T(t.differential_track_width) ? globalThis.Number(t.differential_track_width) : 0,
      bumper: T(t.bumper) ? rt.fromJSON(t.bumper) : void 0,
      frontLeft: T(t.frontLeft) ? w.fromJSON(t.frontLeft) : T(t.front_left) ? w.fromJSON(t.front_left) : void 0,
      frontRight: T(t.frontRight) ? w.fromJSON(t.frontRight) : T(t.front_right) ? w.fromJSON(t.front_right) : void 0,
      backLeft: T(t.backLeft) ? w.fromJSON(t.backLeft) : T(t.back_left) ? w.fromJSON(t.back_left) : void 0,
      backRight: T(t.backRight) ? w.fromJSON(t.backRight) : T(t.back_right) ? w.fromJSON(t.back_right) : void 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.mass !== 0 && (e.mass = t.mass), t.inertia !== 0 && (e.inertia = t.inertia), t.gearing !== 0 && (e.gearing = t.gearing), t.radius !== 0 && (e.radius = t.radius), t.vmax !== 0 && (e.vmax = t.vmax), t.tmax !== 0 && (e.tmax = t.tmax), t.cof !== 0 && (e.cof = t.cof), t.differentialTrackWidth !== 0 && (e.differentialTrackWidth = t.differentialTrackWidth), t.bumper !== void 0 && (e.bumper = rt.toJSON(t.bumper)), t.frontLeft !== void 0 && (e.frontLeft = w.toJSON(t.frontLeft)), t.frontRight !== void 0 && (e.frontRight = w.toJSON(t.frontRight)), t.backLeft !== void 0 && (e.backLeft = w.toJSON(t.backLeft)), t.backRight !== void 0 && (e.backRight = w.toJSON(t.backRight)), e;
  },
  create(t) {
    return nt.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = se();
    return e.mass = t.mass ?? 0, e.inertia = t.inertia ?? 0, e.gearing = t.gearing ?? 0, e.radius = t.radius ?? 0, e.vmax = t.vmax ?? 0, e.tmax = t.tmax ?? 0, e.cof = t.cof ?? 0, e.differentialTrackWidth = t.differentialTrackWidth ?? 0, e.bumper = t.bumper !== void 0 && t.bumper !== null ? rt.fromPartial(t.bumper) : void 0, e.frontLeft = t.frontLeft !== void 0 && t.frontLeft !== null ? w.fromPartial(t.frontLeft) : void 0, e.frontRight = t.frontRight !== void 0 && t.frontRight !== null ? w.fromPartial(t.frontRight) : void 0, e.backLeft = t.backLeft !== void 0 && t.backLeft !== null ? w.fromPartial(t.backLeft) : void 0, e.backRight = t.backRight !== void 0 && t.backRight !== null ? w.fromPartial(t.backRight) : void 0, e;
  }
};
function T(t) {
  return t != null;
}
function fe() {
  return { x: void 0, y: void 0 };
}
const O = {
  encode(t, e = new S()) {
    return t.x !== void 0 && y.encode(t.x, e.uint32(10).fork()).join(), t.y !== void 0 && y.encode(t.y, e.uint32(18).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = fe();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.x = y.decode(r, r.uint32());
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.y = y.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      x: E(t.x) ? y.fromJSON(t.x) : void 0,
      y: E(t.y) ? y.fromJSON(t.y) : void 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.x !== void 0 && (e.x = y.toJSON(t.x)), t.y !== void 0 && (e.y = y.toJSON(t.y)), e;
  },
  create(t) {
    return O.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = fe();
    return e.x = t.x !== void 0 && t.x !== null ? y.fromPartial(t.x) : void 0, e.y = t.y !== void 0 && t.y !== null ? y.fromPartial(t.y) : void 0, e;
  }
};
function ue() {
  return { front: void 0, left: void 0, right: void 0, back: void 0 };
}
const it = {
  encode(t, e = new S()) {
    return t.front !== void 0 && y.encode(t.front, e.uint32(10).fork()).join(), t.left !== void 0 && y.encode(t.left, e.uint32(18).fork()).join(), t.right !== void 0 && y.encode(t.right, e.uint32(26).fork()).join(), t.back !== void 0 && y.encode(t.back, e.uint32(34).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = ue();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.front = y.decode(r, r.uint32());
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.left = y.decode(r, r.uint32());
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.right = y.decode(r, r.uint32());
          continue;
        }
        case 4: {
          if (n !== 34)
            break;
          i.back = y.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      front: E(t.front) ? y.fromJSON(t.front) : void 0,
      left: E(t.left) ? y.fromJSON(t.left) : void 0,
      right: E(t.right) ? y.fromJSON(t.right) : void 0,
      back: E(t.back) ? y.fromJSON(t.back) : void 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.front !== void 0 && (e.front = y.toJSON(t.front)), t.left !== void 0 && (e.left = y.toJSON(t.left)), t.right !== void 0 && (e.right = y.toJSON(t.right)), t.back !== void 0 && (e.back = y.toJSON(t.back)), e;
  },
  create(t) {
    return it.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = ue();
    return e.front = t.front !== void 0 && t.front !== null ? y.fromPartial(t.front) : void 0, e.left = t.left !== void 0 && t.left !== null ? y.fromPartial(t.left) : void 0, e.right = t.right !== void 0 && t.right !== null ? y.fromPartial(t.right) : void 0, e.back = t.back !== void 0 && t.back !== null ? y.fromPartial(t.back) : void 0, e;
  }
};
function de() {
  return {
    mass: void 0,
    inertia: void 0,
    gearing: void 0,
    radius: void 0,
    vmax: void 0,
    tmax: void 0,
    cof: void 0,
    differentialTrackWidth: void 0,
    bumper: void 0,
    frontLeft: void 0,
    frontRight: void 0,
    backLeft: void 0,
    backRight: void 0
  };
}
const ot = {
  encode(t, e = new S()) {
    return t.mass !== void 0 && y.encode(t.mass, e.uint32(10).fork()).join(), t.inertia !== void 0 && y.encode(t.inertia, e.uint32(18).fork()).join(), t.gearing !== void 0 && y.encode(t.gearing, e.uint32(26).fork()).join(), t.radius !== void 0 && y.encode(t.radius, e.uint32(34).fork()).join(), t.vmax !== void 0 && y.encode(t.vmax, e.uint32(42).fork()).join(), t.tmax !== void 0 && y.encode(t.tmax, e.uint32(50).fork()).join(), t.cof !== void 0 && y.encode(t.cof, e.uint32(58).fork()).join(), t.differentialTrackWidth !== void 0 && y.encode(t.differentialTrackWidth, e.uint32(66).fork()).join(), t.bumper !== void 0 && it.encode(t.bumper, e.uint32(74).fork()).join(), t.frontLeft !== void 0 && O.encode(t.frontLeft, e.uint32(82).fork()).join(), t.frontRight !== void 0 && O.encode(t.frontRight, e.uint32(90).fork()).join(), t.backLeft !== void 0 && O.encode(t.backLeft, e.uint32(98).fork()).join(), t.backRight !== void 0 && O.encode(t.backRight, e.uint32(106).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = de();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.mass = y.decode(r, r.uint32());
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.inertia = y.decode(r, r.uint32());
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.gearing = y.decode(r, r.uint32());
          continue;
        }
        case 4: {
          if (n !== 34)
            break;
          i.radius = y.decode(r, r.uint32());
          continue;
        }
        case 5: {
          if (n !== 42)
            break;
          i.vmax = y.decode(r, r.uint32());
          continue;
        }
        case 6: {
          if (n !== 50)
            break;
          i.tmax = y.decode(r, r.uint32());
          continue;
        }
        case 7: {
          if (n !== 58)
            break;
          i.cof = y.decode(r, r.uint32());
          continue;
        }
        case 8: {
          if (n !== 66)
            break;
          i.differentialTrackWidth = y.decode(r, r.uint32());
          continue;
        }
        case 9: {
          if (n !== 74)
            break;
          i.bumper = it.decode(r, r.uint32());
          continue;
        }
        case 10: {
          if (n !== 82)
            break;
          i.frontLeft = O.decode(r, r.uint32());
          continue;
        }
        case 11: {
          if (n !== 90)
            break;
          i.frontRight = O.decode(r, r.uint32());
          continue;
        }
        case 12: {
          if (n !== 98)
            break;
          i.backLeft = O.decode(r, r.uint32());
          continue;
        }
        case 13: {
          if (n !== 106)
            break;
          i.backRight = O.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      mass: E(t.mass) ? y.fromJSON(t.mass) : void 0,
      inertia: E(t.inertia) ? y.fromJSON(t.inertia) : void 0,
      gearing: E(t.gearing) ? y.fromJSON(t.gearing) : void 0,
      radius: E(t.radius) ? y.fromJSON(t.radius) : void 0,
      vmax: E(t.vmax) ? y.fromJSON(t.vmax) : void 0,
      tmax: E(t.tmax) ? y.fromJSON(t.tmax) : void 0,
      cof: E(t.cof) ? y.fromJSON(t.cof) : void 0,
      differentialTrackWidth: E(t.differentialTrackWidth) ? y.fromJSON(t.differentialTrackWidth) : E(t.differential_track_width) ? y.fromJSON(t.differential_track_width) : void 0,
      bumper: E(t.bumper) ? it.fromJSON(t.bumper) : void 0,
      frontLeft: E(t.frontLeft) ? O.fromJSON(t.frontLeft) : E(t.front_left) ? O.fromJSON(t.front_left) : void 0,
      frontRight: E(t.frontRight) ? O.fromJSON(t.frontRight) : E(t.front_right) ? O.fromJSON(t.front_right) : void 0,
      backLeft: E(t.backLeft) ? O.fromJSON(t.backLeft) : E(t.back_left) ? O.fromJSON(t.back_left) : void 0,
      backRight: E(t.backRight) ? O.fromJSON(t.backRight) : E(t.back_right) ? O.fromJSON(t.back_right) : void 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.mass !== void 0 && (e.mass = y.toJSON(t.mass)), t.inertia !== void 0 && (e.inertia = y.toJSON(t.inertia)), t.gearing !== void 0 && (e.gearing = y.toJSON(t.gearing)), t.radius !== void 0 && (e.radius = y.toJSON(t.radius)), t.vmax !== void 0 && (e.vmax = y.toJSON(t.vmax)), t.tmax !== void 0 && (e.tmax = y.toJSON(t.tmax)), t.cof !== void 0 && (e.cof = y.toJSON(t.cof)), t.differentialTrackWidth !== void 0 && (e.differentialTrackWidth = y.toJSON(t.differentialTrackWidth)), t.bumper !== void 0 && (e.bumper = it.toJSON(t.bumper)), t.frontLeft !== void 0 && (e.frontLeft = O.toJSON(t.frontLeft)), t.frontRight !== void 0 && (e.frontRight = O.toJSON(t.frontRight)), t.backLeft !== void 0 && (e.backLeft = O.toJSON(t.backLeft)), t.backRight !== void 0 && (e.backRight = O.toJSON(t.backRight)), e;
  },
  create(t) {
    return ot.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = de();
    return e.mass = t.mass !== void 0 && t.mass !== null ? y.fromPartial(t.mass) : void 0, e.inertia = t.inertia !== void 0 && t.inertia !== null ? y.fromPartial(t.inertia) : void 0, e.gearing = t.gearing !== void 0 && t.gearing !== null ? y.fromPartial(t.gearing) : void 0, e.radius = t.radius !== void 0 && t.radius !== null ? y.fromPartial(t.radius) : void 0, e.vmax = t.vmax !== void 0 && t.vmax !== null ? y.fromPartial(t.vmax) : void 0, e.tmax = t.tmax !== void 0 && t.tmax !== null ? y.fromPartial(t.tmax) : void 0, e.cof = t.cof !== void 0 && t.cof !== null ? y.fromPartial(t.cof) : void 0, e.differentialTrackWidth = t.differentialTrackWidth !== void 0 && t.differentialTrackWidth !== null ? y.fromPartial(t.differentialTrackWidth) : void 0, e.bumper = t.bumper !== void 0 && t.bumper !== null ? it.fromPartial(t.bumper) : void 0, e.frontLeft = t.frontLeft !== void 0 && t.frontLeft !== null ? O.fromPartial(t.frontLeft) : void 0, e.frontRight = t.frontRight !== void 0 && t.frontRight !== null ? O.fromPartial(t.frontRight) : void 0, e.backLeft = t.backLeft !== void 0 && t.backLeft !== null ? O.fromPartial(t.backLeft) : void 0, e.backRight = t.backRight !== void 0 && t.backRight !== null ? O.fromPartial(t.backRight) : void 0, e;
  }
};
function E(t) {
  return t != null;
}
const cr = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DoubleBumper: rt,
  DoubleModule: w,
  DoubleRobotConfig: nt,
  ExprBumper: it,
  ExprModule: O,
  ExprRobotConfig: ot
}, Symbol.toStringTag, { value: "Module" })), hr = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DoubleParameters: tt,
  ExprParameters: et,
  WaypointID: P,
  WaypointIDFirst: X,
  WaypointIDLast: q,
  WaypointIDX: K,
  constraint: fr,
  robotconfig: cr,
  waypoint: lr
}, Symbol.toStringTag, { value: "Module" }));
function le() {
  return { name: "", config: void 0, driveType: 0 };
}
const De = {
  encode(t, e = new S()) {
    return t.name !== "" && e.uint32(10).string(t.name), t.config !== void 0 && ot.encode(t.config, e.uint32(18).fork()).join(), t.driveType !== 0 && e.uint32(24).int32(t.driveType), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = le();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.name = r.string();
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.config = ot.decode(r, r.uint32());
          continue;
        }
        case 3: {
          if (n !== 24)
            break;
          i.driveType = r.int32();
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      name: ut(t.name) ? globalThis.String(t.name) : "",
      config: ut(t.config) ? ot.fromJSON(t.config) : void 0,
      driveType: ut(t.driveType) ? Ot(t.driveType) : ut(t.drive_type) ? Ot(t.drive_type) : 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.name !== "" && (e.name = t.name), t.config !== void 0 && (e.config = ot.toJSON(t.config)), t.driveType !== 0 && (e.driveType = Ae(t.driveType)), e;
  },
  create(t) {
    return De.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = le();
    return e.name = t.name ?? "", e.config = t.config !== void 0 && t.config !== null ? ot.fromPartial(t.config) : void 0, e.driveType = t.driveType ?? 0, e;
  }
};
function ut(t) {
  return t != null;
}
function ce() {
  return { x: 0, y: 0 };
}
const _ = {
  encode(t, e = new S()) {
    return t.x !== 0 && e.uint32(9).double(t.x), t.y !== 0 && e.uint32(17).double(t.y), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = ce();
    for (; r.pos < o; ) {
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
  fromJSON(t) {
    return {
      x: A(t.x) ? globalThis.Number(t.x) : 0,
      y: A(t.y) ? globalThis.Number(t.y) : 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.x !== 0 && (e.x = t.x), t.y !== 0 && (e.y = t.y), e;
  },
  create(t) {
    return _.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = ce();
    return e.x = t.x ?? 0, e.y = t.y ?? 0, e;
  }
};
function he() {
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
const J = {
  encode(t, e = new S()) {
    return t.t !== 0 && e.uint32(9).double(t.t), t.x !== 0 && e.uint32(17).double(t.x), t.y !== 0 && e.uint32(25).double(t.y), t.heading !== 0 && e.uint32(33).double(t.heading), t.vx !== 0 && e.uint32(41).double(t.vx), t.vy !== 0 && e.uint32(49).double(t.vy), t.omega !== 0 && e.uint32(57).double(t.omega), t.ax !== 0 && e.uint32(65).double(t.ax), t.ay !== 0 && e.uint32(73).double(t.ay), t.alpha !== 0 && e.uint32(81).double(t.alpha), t.fl !== void 0 && _.encode(t.fl, e.uint32(90).fork()).join(), t.fr !== void 0 && _.encode(t.fr, e.uint32(98).fork()).join(), t.bl !== void 0 && _.encode(t.bl, e.uint32(106).fork()).join(), t.br !== void 0 && _.encode(t.br, e.uint32(114).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = he();
    for (; r.pos < o; ) {
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
          i.fl = _.decode(r, r.uint32());
          continue;
        }
        case 12: {
          if (n !== 98)
            break;
          i.fr = _.decode(r, r.uint32());
          continue;
        }
        case 13: {
          if (n !== 106)
            break;
          i.bl = _.decode(r, r.uint32());
          continue;
        }
        case 14: {
          if (n !== 114)
            break;
          i.br = _.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      t: A(t.t) ? globalThis.Number(t.t) : 0,
      x: A(t.x) ? globalThis.Number(t.x) : 0,
      y: A(t.y) ? globalThis.Number(t.y) : 0,
      heading: A(t.heading) ? globalThis.Number(t.heading) : 0,
      vx: A(t.vx) ? globalThis.Number(t.vx) : 0,
      vy: A(t.vy) ? globalThis.Number(t.vy) : 0,
      omega: A(t.omega) ? globalThis.Number(t.omega) : 0,
      ax: A(t.ax) ? globalThis.Number(t.ax) : 0,
      ay: A(t.ay) ? globalThis.Number(t.ay) : 0,
      alpha: A(t.alpha) ? globalThis.Number(t.alpha) : 0,
      fl: A(t.fl) ? _.fromJSON(t.fl) : void 0,
      fr: A(t.fr) ? _.fromJSON(t.fr) : void 0,
      bl: A(t.bl) ? _.fromJSON(t.bl) : void 0,
      br: A(t.br) ? _.fromJSON(t.br) : void 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.t !== 0 && (e.t = t.t), t.x !== 0 && (e.x = t.x), t.y !== 0 && (e.y = t.y), t.heading !== 0 && (e.heading = t.heading), t.vx !== 0 && (e.vx = t.vx), t.vy !== 0 && (e.vy = t.vy), t.omega !== 0 && (e.omega = t.omega), t.ax !== 0 && (e.ax = t.ax), t.ay !== 0 && (e.ay = t.ay), t.alpha !== 0 && (e.alpha = t.alpha), t.fl !== void 0 && (e.fl = _.toJSON(t.fl)), t.fr !== void 0 && (e.fr = _.toJSON(t.fr)), t.bl !== void 0 && (e.bl = _.toJSON(t.bl)), t.br !== void 0 && (e.br = _.toJSON(t.br)), e;
  },
  create(t) {
    return J.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = he();
    return e.t = t.t ?? 0, e.x = t.x ?? 0, e.y = t.y ?? 0, e.heading = t.heading ?? 0, e.vx = t.vx ?? 0, e.vy = t.vy ?? 0, e.omega = t.omega ?? 0, e.ax = t.ax ?? 0, e.ay = t.ay ?? 0, e.alpha = t.alpha ?? 0, e.fl = t.fl !== void 0 && t.fl !== null ? _.fromPartial(t.fl) : void 0, e.fr = t.fr !== void 0 && t.fr !== null ? _.fromPartial(t.fr) : void 0, e.bl = t.bl !== void 0 && t.bl !== null ? _.fromPartial(t.bl) : void 0, e.br = t.br !== void 0 && t.br !== null ? _.fromPartial(t.br) : void 0, e;
  }
};
function A(t) {
  return t != null;
}
function pe() {
  return { samples: [] };
}
const at = {
  encode(t, e = new S()) {
    for (const r of t.samples)
      J.encode(r, e.uint32(10).fork()).join();
    return e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = pe();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.samples.push(J.decode(r, r.uint32()));
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      samples: globalThis.Array.isArray(t?.samples) ? t.samples.map((e) => J.fromJSON(e)) : []
    };
  },
  toJSON(t) {
    const e = {};
    return t.samples?.length && (e.samples = t.samples.map((r) => J.toJSON(r))), e;
  },
  create(t) {
    return at.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = pe();
    return e.samples = t.samples?.map((r) => J.fromPartial(r)) || [], e;
  }
};
function ve() {
  return { samples: [] };
}
const st = {
  encode(t, e = new S()) {
    for (const r of t.samples)
      W.encode(r, e.uint32(10).fork()).join();
    return e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = ve();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.samples.push(W.decode(r, r.uint32()));
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      samples: globalThis.Array.isArray(t?.samples) ? t.samples.map((e) => W.fromJSON(e)) : []
    };
  },
  toJSON(t) {
    const e = {};
    return t.samples?.length && (e.samples = t.samples.map((r) => W.toJSON(r))), e;
  },
  create(t) {
    return st.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = ve();
    return e.samples = t.samples?.map((r) => W.fromPartial(r)) || [], e;
  }
};
function ye() {
  return { trajectory: void 0, splits: [], waypoints: [], config: void 0 };
}
const ft = {
  encode(t, e = new S()) {
    switch (t.trajectory?.$case) {
      case "swerve":
        at.encode(t.trajectory.swerve, e.uint32(10).fork()).join();
        break;
      case "differential":
        st.encode(t.trajectory.differential, e.uint32(18).fork()).join();
        break;
    }
    e.uint32(26).fork();
    for (const r of t.splits)
      e.uint64(r);
    e.join(), e.uint32(34).fork();
    for (const r of t.waypoints)
      e.double(r);
    return e.join(), t.config !== void 0 && nt.encode(t.config, e.uint32(42).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = ye();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.trajectory = { $case: "swerve", swerve: at.decode(r, r.uint32()) };
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.trajectory = {
            $case: "differential",
            differential: st.decode(r, r.uint32())
          };
          continue;
        }
        case 3: {
          if (n === 24) {
            i.splits.push(me(r.uint64()));
            continue;
          }
          if (n === 26) {
            const f = r.uint32() + r.pos;
            for (; r.pos < f; )
              i.splits.push(me(r.uint64()));
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
            const f = r.uint32() + r.pos;
            for (; r.pos < f; )
              i.waypoints.push(r.double());
            continue;
          }
          break;
        }
        case 5: {
          if (n !== 42)
            break;
          i.config = nt.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      trajectory: wt(t.swerve) ? { $case: "swerve", swerve: at.fromJSON(t.swerve) } : wt(t.differential) ? { $case: "differential", differential: st.fromJSON(t.differential) } : void 0,
      splits: globalThis.Array.isArray(t?.splits) ? t.splits.map((e) => globalThis.Number(e)) : [],
      waypoints: globalThis.Array.isArray(t?.waypoints) ? t.waypoints.map((e) => globalThis.Number(e)) : [],
      config: wt(t.config) ? nt.fromJSON(t.config) : void 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.trajectory?.$case === "swerve" ? e.swerve = at.toJSON(t.trajectory.swerve) : t.trajectory?.$case === "differential" && (e.differential = st.toJSON(t.trajectory.differential)), t.splits?.length && (e.splits = t.splits.map((r) => Math.round(r))), t.waypoints?.length && (e.waypoints = t.waypoints), t.config !== void 0 && (e.config = nt.toJSON(t.config)), e;
  },
  create(t) {
    return ft.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = ye();
    switch (t.trajectory?.$case) {
      case "swerve": {
        t.trajectory?.swerve !== void 0 && t.trajectory?.swerve !== null && (e.trajectory = { $case: "swerve", swerve: at.fromPartial(t.trajectory.swerve) });
        break;
      }
      case "differential": {
        t.trajectory?.differential !== void 0 && t.trajectory?.differential !== null && (e.trajectory = {
          $case: "differential",
          differential: st.fromPartial(t.trajectory.differential)
        });
        break;
      }
    }
    return e.splits = t.splits?.map((r) => r) || [], e.waypoints = t.waypoints?.map((r) => r) || [], e.config = t.config !== void 0 && t.config !== null ? nt.fromPartial(t.config) : void 0, e;
  }
};
function me(t) {
  const e = globalThis.Number(t.toString());
  if (e > globalThis.Number.MAX_SAFE_INTEGER)
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  if (e < globalThis.Number.MIN_SAFE_INTEGER)
    throw new globalThis.Error("Value is smaller than Number.MIN_SAFE_INTEGER");
  return e;
}
function wt(t) {
  return t != null;
}
function be() {
  return { name: "", params: void 0, snapshot: void 0, trajectory: void 0 };
}
const M = {
  encode(t, e = new S()) {
    return t.name !== "" && e.uint32(10).string(t.name), t.params !== void 0 && et.encode(t.params, e.uint32(18).fork()).join(), t.snapshot !== void 0 && tt.encode(t.snapshot, e.uint32(26).fork()).join(), t.trajectory !== void 0 && ft.encode(t.trajectory, e.uint32(34).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = be();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.name = r.string();
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.params = et.decode(r, r.uint32());
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.snapshot = tt.decode(r, r.uint32());
          continue;
        }
        case 4: {
          if (n !== 34)
            break;
          i.trajectory = ft.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {
      name: dt(t.name) ? globalThis.String(t.name) : "",
      params: dt(t.params) ? et.fromJSON(t.params) : void 0,
      snapshot: dt(t.snapshot) ? tt.fromJSON(t.snapshot) : void 0,
      trajectory: dt(t.trajectory) ? ft.fromJSON(t.trajectory) : void 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.name !== "" && (e.name = t.name), t.params !== void 0 && (e.params = et.toJSON(t.params)), t.snapshot !== void 0 && (e.snapshot = tt.toJSON(t.snapshot)), t.trajectory !== void 0 && (e.trajectory = ft.toJSON(t.trajectory)), e;
  },
  create(t) {
    return M.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = be();
    return e.name = t.name ?? "", e.params = t.params !== void 0 && t.params !== null ? et.fromPartial(t.params) : void 0, e.snapshot = t.snapshot !== void 0 && t.snapshot !== null ? tt.fromPartial(t.snapshot) : void 0, e.trajectory = t.trajectory !== void 0 && t.trajectory !== null ? ft.fromPartial(t.trajectory) : void 0, e;
  }
};
function dt(t) {
  return t != null;
}
const wr = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DifferentialSample: W,
  DifferentialTrajectory: st,
  DriveType: Me,
  Expr: y,
  ForceVector: _,
  GenerationOutput: ft,
  ProjectFile: De,
  SwerveSample: J,
  SwerveTrajectory: at,
  TrajectoryFile: M,
  driveTypeFromJSON: Ot,
  driveTypeToJSON: Ae,
  parameters: hr
}, Symbol.toStringTag, { value: "Module" }));
function xe() {
  return { sample: void 0 };
}
const vt = {
  encode(t, e = new S()) {
    return t.sample !== void 0 && J.encode(t.sample, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = xe();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.sample = J.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return { sample: Ce(t.sample) ? J.fromJSON(t.sample) : void 0 };
  },
  toJSON(t) {
    const e = {};
    return t.sample !== void 0 && (e.sample = J.toJSON(t.sample)), e;
  },
  create(t) {
    return vt.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = xe();
    return e.sample = t.sample !== void 0 && t.sample !== null ? J.fromPartial(t.sample) : void 0, e;
  }
};
function ke() {
  return { sample: void 0 };
}
const Et = {
  encode(t, e = new S()) {
    return t.sample !== void 0 && J.encode(t.sample, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = ke();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.sample = J.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return { sample: Ce(t.sample) ? J.fromJSON(t.sample) : void 0 };
  },
  toJSON(t) {
    const e = {};
    return t.sample !== void 0 && (e.sample = J.toJSON(t.sample)), e;
  },
  create(t) {
    return Et.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = ke();
    return e.sample = t.sample !== void 0 && t.sample !== null ? J.fromPartial(t.sample) : void 0, e;
  }
};
function Ce(t) {
  return t != null;
}
function ge() {
  return { trajectory: void 0 };
}
const yt = {
  encode(t, e = new S()) {
    return t.trajectory !== void 0 && M.encode(t.trajectory, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = ge();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.trajectory = M.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return { trajectory: Ve(t.trajectory) ? M.fromJSON(t.trajectory) : void 0 };
  },
  toJSON(t) {
    const e = {};
    return t.trajectory !== void 0 && (e.trajectory = M.toJSON(t.trajectory)), e;
  },
  create(t) {
    return yt.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = ge();
    return e.trajectory = t.trajectory !== void 0 && t.trajectory !== null ? M.fromPartial(t.trajectory) : void 0, e;
  }
};
function Se() {
  return { trajectory: void 0 };
}
const _t = {
  encode(t, e = new S()) {
    return t.trajectory !== void 0 && M.encode(t.trajectory, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Se();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.trajectory = M.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return { trajectory: Ve(t.trajectory) ? M.fromJSON(t.trajectory) : void 0 };
  },
  toJSON(t) {
    const e = {};
    return t.trajectory !== void 0 && (e.trajectory = M.toJSON(t.trajectory)), e;
  },
  create(t) {
    return _t.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Se();
    return e.trajectory = t.trajectory !== void 0 && t.trajectory !== null ? M.fromPartial(t.trajectory) : void 0, e;
  }
};
function Ve(t) {
  return t != null;
}
function Ne() {
  return { trajectory: void 0 };
}
const Pt = {
  encode(t, e = new S()) {
    return t.trajectory !== void 0 && M.encode(t.trajectory, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Ne();
    for (; r.pos < o; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.trajectory = M.decode(r, r.uint32());
          continue;
        }
      }
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return { trajectory: pr(t.trajectory) ? M.fromJSON(t.trajectory) : void 0 };
  },
  toJSON(t) {
    const e = {};
    return t.trajectory !== void 0 && (e.trajectory = M.toJSON(t.trajectory)), e;
  },
  create(t) {
    return Pt.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Ne();
    return e.trajectory = t.trajectory !== void 0 && t.trajectory !== null ? M.fromPartial(t.trajectory) : void 0, e;
  }
};
function pr(t) {
  return t != null;
}
const vr = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  EchoSwerveSampleRequest: vt,
  EchoSwerveSampleResponse: Et,
  GenerateRequest: yt,
  GenerateResponse: _t,
  GetDefaultTrajectoryResponse: Pt
}, Symbol.toStringTag, { value: "Module" }));
var ht = { exports: {} }, yr = ht.exports, we;
function mr() {
  return we || (we = 1, (function(t, e) {
    (function(r, o) {
      t.exports = o();
    })(yr, (function() {
      return r = { 418: function(i, n) {
        (function(f, s) {
          for (var m in s) f[m] = s[m];
        })(n, (function(f) {
          var s = {};
          function m(u) {
            if (s[u]) return s[u].exports;
            var l = s[u] = { i: u, l: !1, exports: {} };
            return f[u].call(l.exports, l, l.exports, m), l.l = !0, l.exports;
          }
          return m.m = f, m.c = s, m.i = function(u) {
            return u;
          }, m.d = function(u, l, b) {
            m.o(u, l) || Object.defineProperty(u, l, { configurable: !1, enumerable: !0, get: b });
          }, m.n = function(u) {
            var l = u && u.__esModule ? function() {
              return u.default;
            } : function() {
              return u;
            };
            return m.d(l, "a", l), l;
          }, m.o = function(u, l) {
            return Object.prototype.hasOwnProperty.call(u, l);
          }, m.p = "", m(m.s = 1);
        })([function(f, s, m) {
          Object.defineProperty(s, "__esModule", { value: !0 });
          var u = m(3), l = (function() {
            function b(d, h) {
              d === void 0 && (d = {}), h === void 0 && (h = { splitValues: !1 });
              var p, v = this;
              this.headersMap = {}, d && (typeof Headers < "u" && d instanceof Headers ? u.getHeaderKeys(d).forEach((function(a) {
                u.getHeaderValues(d, a).forEach((function(c) {
                  h.splitValues ? v.append(a, u.splitHeaderValue(c)) : v.append(a, c);
                }));
              })) : typeof (p = d) == "object" && typeof p.headersMap == "object" && typeof p.forEach == "function" ? d.forEach((function(a, c) {
                v.append(a, c);
              })) : typeof Map < "u" && d instanceof Map ? d.forEach((function(a, c) {
                v.append(c, a);
              })) : typeof d == "string" ? this.appendFromString(d) : typeof d == "object" && Object.getOwnPropertyNames(d).forEach((function(a) {
                var c = d[a];
                Array.isArray(c) ? c.forEach((function(x) {
                  v.append(a, x);
                })) : v.append(a, c);
              })));
            }
            return b.prototype.appendFromString = function(d) {
              for (var h = d.split(`\r
`), p = 0; p < h.length; p++) {
                var v = h[p], a = v.indexOf(":");
                if (a > 0) {
                  var c = v.substring(0, a).trim(), x = v.substring(a + 1).trim();
                  this.append(c, x);
                }
              }
            }, b.prototype.delete = function(d, h) {
              var p = u.normalizeName(d);
              if (h === void 0) delete this.headersMap[p];
              else {
                var v = this.headersMap[p];
                if (v) {
                  var a = v.indexOf(h);
                  a >= 0 && v.splice(a, 1), v.length === 0 && delete this.headersMap[p];
                }
              }
            }, b.prototype.append = function(d, h) {
              var p = this, v = u.normalizeName(d);
              Array.isArray(this.headersMap[v]) || (this.headersMap[v] = []), Array.isArray(h) ? h.forEach((function(a) {
                p.headersMap[v].push(u.normalizeValue(a));
              })) : this.headersMap[v].push(u.normalizeValue(h));
            }, b.prototype.set = function(d, h) {
              var p = u.normalizeName(d);
              if (Array.isArray(h)) {
                var v = [];
                h.forEach((function(a) {
                  v.push(u.normalizeValue(a));
                })), this.headersMap[p] = v;
              } else this.headersMap[p] = [u.normalizeValue(h)];
            }, b.prototype.has = function(d, h) {
              var p = this.headersMap[u.normalizeName(d)];
              if (!Array.isArray(p)) return !1;
              if (h !== void 0) {
                var v = u.normalizeValue(h);
                return p.indexOf(v) >= 0;
              }
              return !0;
            }, b.prototype.get = function(d) {
              var h = this.headersMap[u.normalizeName(d)];
              return h !== void 0 ? h.concat() : [];
            }, b.prototype.forEach = function(d) {
              var h = this;
              Object.getOwnPropertyNames(this.headersMap).forEach((function(p) {
                d(p, h.headersMap[p]);
              }), this);
            }, b.prototype.toHeaders = function() {
              if (typeof Headers < "u") {
                var d = new Headers();
                return this.forEach((function(h, p) {
                  p.forEach((function(v) {
                    d.append(h, v);
                  }));
                })), d;
              }
              throw new Error("Headers class is not defined");
            }, b;
          })();
          s.BrowserHeaders = l;
        }, function(f, s, m) {
          Object.defineProperty(s, "__esModule", { value: !0 });
          var u = m(0);
          s.BrowserHeaders = u.BrowserHeaders;
        }, function(f, s, m) {
          Object.defineProperty(s, "__esModule", { value: !0 }), s.iterateHeaders = function(u, l) {
            for (var b = u[Symbol.iterator](), d = b.next(); !d.done; ) l(d.value[0]), d = b.next();
          }, s.iterateHeadersKeys = function(u, l) {
            for (var b = u.keys(), d = b.next(); !d.done; ) l(d.value), d = b.next();
          };
        }, function(f, s, m) {
          Object.defineProperty(s, "__esModule", { value: !0 });
          var u = m(2);
          s.normalizeName = function(l) {
            if (typeof l != "string" && (l = String(l)), /[^a-z0-9\-#$%&'*+.\^_`|~]/i.test(l)) throw new TypeError("Invalid character in header field name");
            return l.toLowerCase();
          }, s.normalizeValue = function(l) {
            return typeof l != "string" && (l = String(l)), l;
          }, s.getHeaderValues = function(l, b) {
            var d = l;
            if (d instanceof Headers && d.getAll) return d.getAll(b);
            var h = d.get(b);
            return h && typeof h == "string" ? [h] : h;
          }, s.getHeaderKeys = function(l) {
            var b = l, d = {}, h = [];
            return b.keys ? u.iterateHeadersKeys(b, (function(p) {
              d[p] || (d[p] = !0, h.push(p));
            })) : b.forEach ? b.forEach((function(p, v) {
              d[v] || (d[v] = !0, h.push(v));
            })) : u.iterateHeaders(b, (function(p) {
              var v = p[0];
              d[v] || (d[v] = !0, h.push(v));
            })), h;
          }, s.splitHeaderValue = function(l) {
            var b = [];
            return l.split(", ").forEach((function(d) {
              d.split(",").forEach((function(h) {
                b.push(h);
              }));
            })), b;
          };
        }]));
      }, 617: function(i, n, f) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.ChunkParser = n.ChunkType = n.encodeASCII = n.decodeASCII = void 0;
        var s, m = f(65);
        function u(a) {
          return (c = a) === 9 || c === 10 || c === 13 || a >= 32 && a <= 126;
          var c;
        }
        function l(a) {
          for (var c = 0; c !== a.length; ++c) if (!u(a[c])) throw new Error("Metadata is not valid (printable) ASCII");
          return String.fromCharCode.apply(String, Array.prototype.slice.call(a));
        }
        function b(a) {
          return (128 & a.getUint8(0)) == 128;
        }
        function d(a) {
          return a.getUint32(1, !1);
        }
        function h(a, c, x) {
          return a.byteLength - c >= x;
        }
        function p(a, c, x) {
          if (a.slice) return a.slice(c, x);
          var g = a.length;
          x !== void 0 && (g = x);
          for (var N = new Uint8Array(g - c), R = 0, $ = c; $ < g; $++) N[R++] = a[$];
          return N;
        }
        n.decodeASCII = l, n.encodeASCII = function(a) {
          for (var c = new Uint8Array(a.length), x = 0; x !== a.length; ++x) {
            var g = a.charCodeAt(x);
            if (!u(g)) throw new Error("Metadata contains invalid ASCII");
            c[x] = g;
          }
          return c;
        }, (function(a) {
          a[a.MESSAGE = 1] = "MESSAGE", a[a.TRAILERS = 2] = "TRAILERS";
        })(s = n.ChunkType || (n.ChunkType = {}));
        var v = (function() {
          function a() {
            this.buffer = null, this.position = 0;
          }
          return a.prototype.parse = function(c, x) {
            if (c.length === 0 && x) return [];
            var g, N = [];
            if (this.buffer == null) this.buffer = c, this.position = 0;
            else if (this.position === this.buffer.byteLength) this.buffer = c, this.position = 0;
            else {
              var R = this.buffer.byteLength - this.position, $ = new Uint8Array(R + c.byteLength), ze = p(this.buffer, this.position);
              $.set(ze, 0);
              var Ge = new Uint8Array(c);
              $.set(Ge, R), this.buffer = $, this.position = 0;
            }
            for (; ; ) {
              if (!h(this.buffer, this.position, 5)) return N;
              var xt = p(this.buffer, this.position, this.position + 5), Jt = new DataView(xt.buffer, xt.byteOffset, xt.byteLength), kt = d(Jt);
              if (!h(this.buffer, this.position, 5 + kt)) return N;
              var Mt = p(this.buffer, this.position + 5, this.position + 5 + kt);
              if (this.position += 5 + kt, b(Jt)) return N.push({ chunkType: s.TRAILERS, trailers: (g = Mt, new m.Metadata(l(g))) }), N;
              N.push({ chunkType: s.MESSAGE, data: Mt });
            }
          }, a;
        })();
        n.ChunkParser = v;
      }, 8: function(i, n) {
        var f;
        Object.defineProperty(n, "__esModule", { value: !0 }), n.httpStatusToCode = n.Code = void 0, (function(s) {
          s[s.OK = 0] = "OK", s[s.Canceled = 1] = "Canceled", s[s.Unknown = 2] = "Unknown", s[s.InvalidArgument = 3] = "InvalidArgument", s[s.DeadlineExceeded = 4] = "DeadlineExceeded", s[s.NotFound = 5] = "NotFound", s[s.AlreadyExists = 6] = "AlreadyExists", s[s.PermissionDenied = 7] = "PermissionDenied", s[s.ResourceExhausted = 8] = "ResourceExhausted", s[s.FailedPrecondition = 9] = "FailedPrecondition", s[s.Aborted = 10] = "Aborted", s[s.OutOfRange = 11] = "OutOfRange", s[s.Unimplemented = 12] = "Unimplemented", s[s.Internal = 13] = "Internal", s[s.Unavailable = 14] = "Unavailable", s[s.DataLoss = 15] = "DataLoss", s[s.Unauthenticated = 16] = "Unauthenticated";
        })(f = n.Code || (n.Code = {})), n.httpStatusToCode = function(s) {
          switch (s) {
            case 0:
              return f.Internal;
            case 200:
              return f.OK;
            case 400:
              return f.InvalidArgument;
            case 401:
              return f.Unauthenticated;
            case 403:
              return f.PermissionDenied;
            case 404:
              return f.NotFound;
            case 409:
              return f.Aborted;
            case 412:
              return f.FailedPrecondition;
            case 429:
              return f.ResourceExhausted;
            case 499:
              return f.Canceled;
            case 500:
              return f.Unknown;
            case 501:
              return f.Unimplemented;
            case 503:
              return f.Unavailable;
            case 504:
              return f.DeadlineExceeded;
            default:
              return f.Unknown;
          }
        };
      }, 934: function(i, n, f) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.client = void 0;
        var s = f(65), m = f(617), u = f(8), l = f(346), b = f(57), d = f(882);
        n.client = function(v, a) {
          return new h(v, a);
        };
        var h = (function() {
          function v(a, c) {
            this.started = !1, this.sentFirstMessage = !1, this.completed = !1, this.closed = !1, this.finishedSending = !1, this.onHeadersCallbacks = [], this.onMessageCallbacks = [], this.onEndCallbacks = [], this.parser = new m.ChunkParser(), this.methodDefinition = a, this.props = c, this.createTransport();
          }
          return v.prototype.createTransport = function() {
            var a = this.props.host + "/" + this.methodDefinition.service.serviceName + "/" + this.methodDefinition.methodName, c = { methodDefinition: this.methodDefinition, debug: this.props.debug || !1, url: a, onHeaders: this.onTransportHeaders.bind(this), onChunk: this.onTransportChunk.bind(this), onEnd: this.onTransportEnd.bind(this) };
            this.props.transport ? this.transport = this.props.transport(c) : this.transport = b.makeDefaultTransport(c);
          }, v.prototype.onTransportHeaders = function(a, c) {
            if (this.props.debug && l.debug("onHeaders", a, c), this.closed) this.props.debug && l.debug("grpc.onHeaders received after request was closed - ignoring");
            else if (c !== 0) {
              this.responseHeaders = a, this.props.debug && l.debug("onHeaders.responseHeaders", JSON.stringify(this.responseHeaders, null, 2));
              var x = p(a);
              this.props.debug && l.debug("onHeaders.gRPCStatus", x);
              var g = x && x >= 0 ? x : u.httpStatusToCode(c);
              this.props.debug && l.debug("onHeaders.code", g);
              var N = a.get("grpc-message") || [];
              if (this.props.debug && l.debug("onHeaders.gRPCMessage", N), this.rawOnHeaders(a), g !== u.Code.OK) {
                var R = this.decodeGRPCStatus(N[0]);
                this.rawOnError(g, R, a);
              }
            }
          }, v.prototype.onTransportChunk = function(a) {
            var c = this;
            if (this.closed) this.props.debug && l.debug("grpc.onChunk received after request was closed - ignoring");
            else {
              var x = [];
              try {
                x = this.parser.parse(a);
              } catch (g) {
                return this.props.debug && l.debug("onChunk.parsing error", g, g.message), void this.rawOnError(u.Code.Internal, "parsing error: " + g.message);
              }
              x.forEach((function(g) {
                if (g.chunkType === m.ChunkType.MESSAGE) {
                  var N = c.methodDefinition.responseType.deserializeBinary(g.data);
                  c.rawOnMessage(N);
                } else g.chunkType === m.ChunkType.TRAILERS && (c.responseHeaders ? (c.responseTrailers = new s.Metadata(g.trailers), c.props.debug && l.debug("onChunk.trailers", c.responseTrailers)) : (c.responseHeaders = new s.Metadata(g.trailers), c.rawOnHeaders(c.responseHeaders)));
              }));
            }
          }, v.prototype.onTransportEnd = function() {
            if (this.props.debug && l.debug("grpc.onEnd"), this.closed) this.props.debug && l.debug("grpc.onEnd received after request was closed - ignoring");
            else if (this.responseTrailers !== void 0) {
              var a = p(this.responseTrailers);
              if (a !== null) {
                var c = this.responseTrailers.get("grpc-message"), x = this.decodeGRPCStatus(c[0]);
                this.rawOnEnd(a, x, this.responseTrailers);
              } else this.rawOnError(u.Code.Internal, "Response closed without grpc-status (Trailers provided)");
            } else {
              if (this.responseHeaders === void 0) return void this.rawOnError(u.Code.Unknown, "Response closed without headers");
              var g = p(this.responseHeaders), N = this.responseHeaders.get("grpc-message");
              if (this.props.debug && l.debug("grpc.headers only response ", g, N), g === null) return void this.rawOnEnd(u.Code.Unknown, "Response closed without grpc-status (Headers only)", this.responseHeaders);
              var R = this.decodeGRPCStatus(N[0]);
              this.rawOnEnd(g, R, this.responseHeaders);
            }
          }, v.prototype.decodeGRPCStatus = function(a) {
            if (!a) return "";
            try {
              return decodeURIComponent(a);
            } catch {
              return a;
            }
          }, v.prototype.rawOnEnd = function(a, c, x) {
            var g = this;
            this.props.debug && l.debug("rawOnEnd", a, c, x), this.completed || (this.completed = !0, this.onEndCallbacks.forEach((function(N) {
              if (!g.closed) try {
                N(a, c, x);
              } catch (R) {
                setTimeout((function() {
                  throw R;
                }), 0);
              }
            })));
          }, v.prototype.rawOnHeaders = function(a) {
            this.props.debug && l.debug("rawOnHeaders", a), this.completed || this.onHeadersCallbacks.forEach((function(c) {
              try {
                c(a);
              } catch (x) {
                setTimeout((function() {
                  throw x;
                }), 0);
              }
            }));
          }, v.prototype.rawOnError = function(a, c, x) {
            var g = this;
            x === void 0 && (x = new s.Metadata()), this.props.debug && l.debug("rawOnError", a, c), this.completed || (this.completed = !0, this.onEndCallbacks.forEach((function(N) {
              if (!g.closed) try {
                N(a, c, x);
              } catch (R) {
                setTimeout((function() {
                  throw R;
                }), 0);
              }
            })));
          }, v.prototype.rawOnMessage = function(a) {
            var c = this;
            this.props.debug && l.debug("rawOnMessage", a.toObject()), this.completed || this.closed || this.onMessageCallbacks.forEach((function(x) {
              if (!c.closed) try {
                x(a);
              } catch (g) {
                setTimeout((function() {
                  throw g;
                }), 0);
              }
            }));
          }, v.prototype.onHeaders = function(a) {
            this.onHeadersCallbacks.push(a);
          }, v.prototype.onMessage = function(a) {
            this.onMessageCallbacks.push(a);
          }, v.prototype.onEnd = function(a) {
            this.onEndCallbacks.push(a);
          }, v.prototype.start = function(a) {
            if (this.started) throw new Error("Client already started - cannot .start()");
            this.started = !0;
            var c = new s.Metadata(a || {});
            c.set("content-type", "application/grpc-web+proto"), c.set("x-grpc-web", "1"), this.transport.start(c);
          }, v.prototype.send = function(a) {
            if (!this.started) throw new Error("Client not started - .start() must be called before .send()");
            if (this.closed) throw new Error("Client already closed - cannot .send()");
            if (this.finishedSending) throw new Error("Client already finished sending - cannot .send()");
            if (!this.methodDefinition.requestStream && this.sentFirstMessage) throw new Error("Message already sent for non-client-streaming method - cannot .send()");
            this.sentFirstMessage = !0;
            var c = d.frameRequest(a);
            this.transport.sendMessage(c);
          }, v.prototype.finishSend = function() {
            if (!this.started) throw new Error("Client not started - .finishSend() must be called before .close()");
            if (this.closed) throw new Error("Client already closed - cannot .send()");
            if (this.finishedSending) throw new Error("Client already finished sending - cannot .finishSend()");
            this.finishedSending = !0, this.transport.finishSend();
          }, v.prototype.close = function() {
            if (!this.started) throw new Error("Client not started - .start() must be called before .close()");
            if (this.closed) throw new Error("Client already closed - cannot .close()");
            this.closed = !0, this.props.debug && l.debug("request.abort aborting request"), this.transport.cancel();
          }, v;
        })();
        function p(v) {
          var a = v.get("grpc-status") || [];
          if (a.length > 0) try {
            var c = a[0];
            return parseInt(c, 10);
          } catch {
            return null;
          }
          return null;
        }
      }, 346: function(i, n) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.debug = void 0, n.debug = function() {
          for (var f = [], s = 0; s < arguments.length; s++) f[s] = arguments[s];
          console.debug ? console.debug.apply(null, f) : console.log.apply(null, f);
        };
      }, 607: function(i, n, f) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.grpc = void 0;
        var s, m = f(418), u = f(57), l = f(229), b = f(540), d = f(210), h = f(859), p = f(8), v = f(938), a = f(35), c = f(934);
        (s = n.grpc || (n.grpc = {})).setDefaultTransport = u.setDefaultTransportFactory, s.CrossBrowserHttpTransport = h.CrossBrowserHttpTransport, s.FetchReadableStreamTransport = l.FetchReadableStreamTransport, s.XhrTransport = d.XhrTransport, s.WebsocketTransport = b.WebsocketTransport, s.Code = p.Code, s.Metadata = m.BrowserHeaders, s.client = function(x, g) {
          return c.client(x, g);
        }, s.invoke = v.invoke, s.unary = a.unary;
      }, 938: function(i, n, f) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.invoke = void 0;
        var s = f(934);
        n.invoke = function(m, u) {
          if (m.requestStream) throw new Error(".invoke cannot be used with client-streaming methods. Use .client instead.");
          var l = s.client(m, { host: u.host, transport: u.transport, debug: u.debug });
          return u.onHeaders && l.onHeaders(u.onHeaders), u.onMessage && l.onMessage(u.onMessage), u.onEnd && l.onEnd(u.onEnd), l.start(u.metadata), l.send(u.request), l.finishSend(), { close: function() {
            l.close();
          } };
        };
      }, 65: function(i, n, f) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.Metadata = void 0;
        var s = f(418);
        Object.defineProperty(n, "Metadata", { enumerable: !0, get: function() {
          return s.BrowserHeaders;
        } });
      }, 57: function(i, n, f) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.makeDefaultTransport = n.setDefaultTransportFactory = void 0;
        var s = f(859), m = function(u) {
          return s.CrossBrowserHttpTransport({ withCredentials: !1 })(u);
        };
        n.setDefaultTransportFactory = function(u) {
          m = u;
        }, n.makeDefaultTransport = function(u) {
          return m(u);
        };
      }, 229: function(i, n, f) {
        var s = this && this.__assign || function() {
          return (s = Object.assign || function(b) {
            for (var d, h = 1, p = arguments.length; h < p; h++) for (var v in d = arguments[h]) Object.prototype.hasOwnProperty.call(d, v) && (b[v] = d[v]);
            return b;
          }).apply(this, arguments);
        };
        Object.defineProperty(n, "__esModule", { value: !0 }), n.detectFetchSupport = n.FetchReadableStreamTransport = void 0;
        var m = f(65), u = f(346);
        n.FetchReadableStreamTransport = function(b) {
          return function(d) {
            return (function(h, p) {
              return h.debug && u.debug("fetchRequest", h), new l(h, p);
            })(d, b);
          };
        };
        var l = (function() {
          function b(d, h) {
            this.cancelled = !1, this.controller = self.AbortController && new AbortController(), this.options = d, this.init = h;
          }
          return b.prototype.pump = function(d, h) {
            var p = this;
            if (this.reader = d, this.cancelled) return this.options.debug && u.debug("Fetch.pump.cancel at first pump"), void this.reader.cancel().catch((function(v) {
              p.options.debug && u.debug("Fetch.pump.reader.cancel exception", v);
            }));
            this.reader.read().then((function(v) {
              if (v.done) return p.options.onEnd(), h;
              p.options.onChunk(v.value), p.pump(p.reader, h);
            })).catch((function(v) {
              p.cancelled ? p.options.debug && u.debug("Fetch.catch - request cancelled") : (p.cancelled = !0, p.options.debug && u.debug("Fetch.catch", v.message), p.options.onEnd(v));
            }));
          }, b.prototype.send = function(d) {
            var h = this;
            fetch(this.options.url, s(s({}, this.init), { headers: this.metadata.toHeaders(), method: "POST", body: d, signal: this.controller && this.controller.signal })).then((function(p) {
              if (h.options.debug && u.debug("Fetch.response", p), h.options.onHeaders(new m.Metadata(p.headers), p.status), !p.body) return p;
              h.pump(p.body.getReader(), p);
            })).catch((function(p) {
              h.cancelled ? h.options.debug && u.debug("Fetch.catch - request cancelled") : (h.cancelled = !0, h.options.debug && u.debug("Fetch.catch", p.message), h.options.onEnd(p));
            }));
          }, b.prototype.sendMessage = function(d) {
            this.send(d);
          }, b.prototype.finishSend = function() {
          }, b.prototype.start = function(d) {
            this.metadata = d;
          }, b.prototype.cancel = function() {
            var d = this;
            this.cancelled ? this.options.debug && u.debug("Fetch.cancel already cancelled") : (this.cancelled = !0, this.controller ? (this.options.debug && u.debug("Fetch.cancel.controller.abort"), this.controller.abort()) : this.options.debug && u.debug("Fetch.cancel.missing abort controller"), this.reader ? (this.options.debug && u.debug("Fetch.cancel.reader.cancel"), this.reader.cancel().catch((function(h) {
              d.options.debug && u.debug("Fetch.cancel.reader.cancel exception", h);
            }))) : this.options.debug && u.debug("Fetch.cancel before reader"));
          }, b;
        })();
        n.detectFetchSupport = function() {
          return typeof Response < "u" && Response.prototype.hasOwnProperty("body") && typeof Headers == "function";
        };
      }, 859: function(i, n, f) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.CrossBrowserHttpTransport = void 0;
        var s = f(229), m = f(210);
        n.CrossBrowserHttpTransport = function(u) {
          if (s.detectFetchSupport()) {
            var l = { credentials: u.withCredentials ? "include" : "same-origin" };
            return s.FetchReadableStreamTransport(l);
          }
          return m.XhrTransport({ withCredentials: u.withCredentials });
        };
      }, 210: function(i, n, f) {
        var s, m = this && this.__extends || (s = function(a, c) {
          return (s = Object.setPrototypeOf || { __proto__: [] } instanceof Array && function(x, g) {
            x.__proto__ = g;
          } || function(x, g) {
            for (var N in g) Object.prototype.hasOwnProperty.call(g, N) && (x[N] = g[N]);
          })(a, c);
        }, function(a, c) {
          function x() {
            this.constructor = a;
          }
          s(a, c), a.prototype = c === null ? Object.create(c) : (x.prototype = c.prototype, new x());
        });
        Object.defineProperty(n, "__esModule", { value: !0 }), n.stringToArrayBuffer = n.MozChunkedArrayBufferXHR = n.XHR = n.XhrTransport = void 0;
        var u = f(65), l = f(346), b = f(849);
        n.XhrTransport = function(a) {
          return function(c) {
            if (b.detectMozXHRSupport()) return new h(c, a);
            if (b.detectXHROverrideMimeTypeSupport()) return new d(c, a);
            throw new Error("This environment's XHR implementation cannot support binary transfer.");
          };
        };
        var d = (function() {
          function a(c, x) {
            this.options = c, this.init = x;
          }
          return a.prototype.onProgressEvent = function() {
            this.options.debug && l.debug("XHR.onProgressEvent.length: ", this.xhr.response.length);
            var c = this.xhr.response.substr(this.index);
            this.index = this.xhr.response.length;
            var x = v(c);
            this.options.onChunk(x);
          }, a.prototype.onLoadEvent = function() {
            this.options.debug && l.debug("XHR.onLoadEvent"), this.options.onEnd();
          }, a.prototype.onStateChange = function() {
            this.options.debug && l.debug("XHR.onStateChange", this.xhr.readyState), this.xhr.readyState === XMLHttpRequest.HEADERS_RECEIVED && this.options.onHeaders(new u.Metadata(this.xhr.getAllResponseHeaders()), this.xhr.status);
          }, a.prototype.sendMessage = function(c) {
            this.xhr.send(c);
          }, a.prototype.finishSend = function() {
          }, a.prototype.start = function(c) {
            var x = this;
            this.metadata = c;
            var g = new XMLHttpRequest();
            this.xhr = g, g.open("POST", this.options.url), this.configureXhr(), this.metadata.forEach((function(N, R) {
              g.setRequestHeader(N, R.join(", "));
            })), g.withCredentials = !!this.init.withCredentials, g.addEventListener("readystatechange", this.onStateChange.bind(this)), g.addEventListener("progress", this.onProgressEvent.bind(this)), g.addEventListener("loadend", this.onLoadEvent.bind(this)), g.addEventListener("error", (function(N) {
              x.options.debug && l.debug("XHR.error", N), x.options.onEnd(N.error);
            }));
          }, a.prototype.configureXhr = function() {
            this.xhr.responseType = "text", this.xhr.overrideMimeType("text/plain; charset=x-user-defined");
          }, a.prototype.cancel = function() {
            this.options.debug && l.debug("XHR.abort"), this.xhr.abort();
          }, a;
        })();
        n.XHR = d;
        var h = (function(a) {
          function c() {
            return a !== null && a.apply(this, arguments) || this;
          }
          return m(c, a), c.prototype.configureXhr = function() {
            this.options.debug && l.debug("MozXHR.configureXhr: setting responseType to 'moz-chunked-arraybuffer'"), this.xhr.responseType = "moz-chunked-arraybuffer";
          }, c.prototype.onProgressEvent = function() {
            var x = this.xhr.response;
            this.options.debug && l.debug("MozXHR.onProgressEvent: ", new Uint8Array(x)), this.options.onChunk(new Uint8Array(x));
          }, c;
        })(d);
        function p(a, c) {
          var x = a.charCodeAt(c);
          if (x >= 55296 && x <= 56319) {
            var g = a.charCodeAt(c + 1);
            g >= 56320 && g <= 57343 && (x = 65536 + (x - 55296 << 10) + (g - 56320));
          }
          return x;
        }
        function v(a) {
          for (var c = new Uint8Array(a.length), x = 0, g = 0; g < a.length; g++) {
            var N = String.prototype.codePointAt ? a.codePointAt(g) : p(a, g);
            c[x++] = 255 & N;
          }
          return c;
        }
        n.MozChunkedArrayBufferXHR = h, n.stringToArrayBuffer = v;
      }, 849: function(i, n) {
        var f;
        function s() {
          if (f !== void 0) return f;
          if (XMLHttpRequest) {
            f = new XMLHttpRequest();
            try {
              f.open("GET", "https://localhost");
            } catch {
            }
          }
          return f;
        }
        function m(u) {
          var l = s();
          if (!l) return !1;
          try {
            return l.responseType = u, l.responseType === u;
          } catch {
          }
          return !1;
        }
        Object.defineProperty(n, "__esModule", { value: !0 }), n.detectXHROverrideMimeTypeSupport = n.detectMozXHRSupport = n.xhrSupportsResponseType = void 0, n.xhrSupportsResponseType = m, n.detectMozXHRSupport = function() {
          return typeof XMLHttpRequest < "u" && m("moz-chunked-arraybuffer");
        }, n.detectXHROverrideMimeTypeSupport = function() {
          return typeof XMLHttpRequest < "u" && XMLHttpRequest.prototype.hasOwnProperty("overrideMimeType");
        };
      }, 540: function(i, n, f) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.WebsocketTransport = void 0;
        var s, m = f(346), u = f(617);
        (function(b) {
          b[b.FINISH_SEND = 1] = "FINISH_SEND";
        })(s || (s = {}));
        var l = new Uint8Array([1]);
        n.WebsocketTransport = function() {
          return function(b) {
            return (function(d) {
              d.debug && m.debug("websocketRequest", d);
              var h, p = (function(c) {
                if (c.substr(0, 8) === "https://") return "wss://" + c.substr(8);
                if (c.substr(0, 7) === "http://") return "ws://" + c.substr(7);
                throw new Error("Websocket transport constructed with non-https:// or http:// host.");
              })(d.url), v = [];
              function a(c) {
                if (c === s.FINISH_SEND) h.send(l);
                else {
                  var x = c, g = new Int8Array(x.byteLength + 1);
                  g.set(new Uint8Array([0])), g.set(x, 1), h.send(g);
                }
              }
              return { sendMessage: function(c) {
                h && h.readyState !== h.CONNECTING ? a(c) : v.push(c);
              }, finishSend: function() {
                h && h.readyState !== h.CONNECTING ? a(s.FINISH_SEND) : v.push(s.FINISH_SEND);
              }, start: function(c) {
                (h = new WebSocket(p, ["grpc-websockets"])).binaryType = "arraybuffer", h.onopen = function() {
                  var x;
                  d.debug && m.debug("websocketRequest.onopen"), h.send((x = "", c.forEach((function(g, N) {
                    x += g + ": " + N.join(", ") + `\r
`;
                  })), u.encodeASCII(x))), v.forEach((function(g) {
                    a(g);
                  }));
                }, h.onclose = function(x) {
                  d.debug && m.debug("websocketRequest.onclose", x), d.onEnd();
                }, h.onerror = function(x) {
                  d.debug && m.debug("websocketRequest.onerror", x);
                }, h.onmessage = function(x) {
                  d.onChunk(new Uint8Array(x.data));
                };
              }, cancel: function() {
                d.debug && m.debug("websocket.abort"), h.close();
              } };
            })(b);
          };
        };
      }, 35: function(i, n, f) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.unary = void 0;
        var s = f(65), m = f(934);
        n.unary = function(u, l) {
          if (u.responseStream) throw new Error(".unary cannot be used with server-streaming methods. Use .invoke or .client instead.");
          if (u.requestStream) throw new Error(".unary cannot be used with client-streaming methods. Use .client instead.");
          var b = null, d = null, h = m.client(u, { host: l.host, transport: l.transport, debug: l.debug });
          return h.onHeaders((function(p) {
            b = p;
          })), h.onMessage((function(p) {
            d = p;
          })), h.onEnd((function(p, v, a) {
            l.onEnd({ status: p, statusMessage: v, headers: b || new s.Metadata(), message: d, trailers: a });
          })), h.start(l.metadata), h.send(l.request), h.finishSend(), { close: function() {
            h.close();
          } };
        };
      }, 882: function(i, n) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.frameRequest = void 0, n.frameRequest = function(f) {
          var s = f.serializeBinary(), m = new ArrayBuffer(s.byteLength + 5);
          return new DataView(m, 1, 4).setUint32(0, s.length, !1), new Uint8Array(m, 5).set(s), new Uint8Array(m);
        };
      } }, o = {}, (function i(n) {
        if (o[n]) return o[n].exports;
        var f = o[n] = { exports: {} };
        return r[n].call(f.exports, f, f.exports, i), f.exports;
      })(607);
      var r, o;
    }));
  })(ht)), ht.exports;
}
var Oe = mr(), pt = { exports: {} }, br = pt.exports, Te;
function xr() {
  return Te || (Te = 1, (function(t, e) {
    (function(o, i) {
      t.exports = i();
    })(br, function() {
      return (
        /******/
        (function(r) {
          var o = {};
          function i(n) {
            if (o[n])
              return o[n].exports;
            var f = o[n] = {
              /******/
              i: n,
              /******/
              l: !1,
              /******/
              exports: {}
              /******/
            };
            return r[n].call(f.exports, f, f.exports, i), f.l = !0, f.exports;
          }
          return i.m = r, i.c = o, i.i = function(n) {
            return n;
          }, i.d = function(n, f, s) {
            i.o(n, f) || Object.defineProperty(n, f, {
              /******/
              configurable: !1,
              /******/
              enumerable: !0,
              /******/
              get: s
              /******/
            });
          }, i.n = function(n) {
            var f = n && n.__esModule ? (
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
            return i.d(f, "a", f), f;
          }, i.o = function(n, f) {
            return Object.prototype.hasOwnProperty.call(n, f);
          }, i.p = "", i(i.s = 1);
        })([
          /* 0 */
          /***/
          (function(r, o, i) {
            Object.defineProperty(o, "__esModule", { value: !0 });
            var n = i(3);
            function f(m) {
              return typeof m == "object" && typeof m.headersMap == "object" && typeof m.forEach == "function";
            }
            var s = (function() {
              function m(u, l) {
                u === void 0 && (u = {}), l === void 0 && (l = { splitValues: !1 });
                var b = this;
                if (this.headersMap = {}, u)
                  if (typeof Headers < "u" && u instanceof Headers) {
                    var d = n.getHeaderKeys(u);
                    d.forEach(function(p) {
                      var v = n.getHeaderValues(u, p);
                      v.forEach(function(a) {
                        l.splitValues ? b.append(p, n.splitHeaderValue(a)) : b.append(p, a);
                      });
                    });
                  } else if (f(u))
                    u.forEach(function(p, v) {
                      b.append(p, v);
                    });
                  else if (typeof Map < "u" && u instanceof Map) {
                    var h = u;
                    h.forEach(function(p, v) {
                      b.append(v, p);
                    });
                  } else typeof u == "string" ? this.appendFromString(u) : typeof u == "object" && Object.getOwnPropertyNames(u).forEach(function(p) {
                    var v = u, a = v[p];
                    Array.isArray(a) ? a.forEach(function(c) {
                      b.append(p, c);
                    }) : b.append(p, a);
                  });
              }
              return m.prototype.appendFromString = function(u) {
                for (var l = u.split(`\r
`), b = 0; b < l.length; b++) {
                  var d = l[b], h = d.indexOf(":");
                  if (h > 0) {
                    var p = d.substring(0, h).trim(), v = d.substring(h + 1).trim();
                    this.append(p, v);
                  }
                }
              }, m.prototype.delete = function(u, l) {
                var b = n.normalizeName(u);
                if (l === void 0)
                  delete this.headersMap[b];
                else {
                  var d = this.headersMap[b];
                  if (d) {
                    var h = d.indexOf(l);
                    h >= 0 && d.splice(h, 1), d.length === 0 && delete this.headersMap[b];
                  }
                }
              }, m.prototype.append = function(u, l) {
                var b = this, d = n.normalizeName(u);
                Array.isArray(this.headersMap[d]) || (this.headersMap[d] = []), Array.isArray(l) ? l.forEach(function(h) {
                  b.headersMap[d].push(n.normalizeValue(h));
                }) : this.headersMap[d].push(n.normalizeValue(l));
              }, m.prototype.set = function(u, l) {
                var b = n.normalizeName(u);
                if (Array.isArray(l)) {
                  var d = [];
                  l.forEach(function(h) {
                    d.push(n.normalizeValue(h));
                  }), this.headersMap[b] = d;
                } else
                  this.headersMap[b] = [n.normalizeValue(l)];
              }, m.prototype.has = function(u, l) {
                var b = this.headersMap[n.normalizeName(u)], d = Array.isArray(b);
                if (!d)
                  return !1;
                if (l !== void 0) {
                  var h = n.normalizeValue(l);
                  return b.indexOf(h) >= 0;
                } else
                  return !0;
              }, m.prototype.get = function(u) {
                var l = this.headersMap[n.normalizeName(u)];
                return l !== void 0 ? l.concat() : [];
              }, m.prototype.forEach = function(u) {
                var l = this;
                Object.getOwnPropertyNames(this.headersMap).forEach(function(b) {
                  u(b, l.headersMap[b]);
                }, this);
              }, m.prototype.toHeaders = function() {
                if (typeof Headers < "u") {
                  var u = new Headers();
                  return this.forEach(function(l, b) {
                    b.forEach(function(d) {
                      u.append(l, d);
                    });
                  }), u;
                } else
                  throw new Error("Headers class is not defined");
              }, m;
            })();
            o.BrowserHeaders = s;
          }),
          /* 1 */
          /***/
          (function(r, o, i) {
            Object.defineProperty(o, "__esModule", { value: !0 });
            var n = i(0);
            o.BrowserHeaders = n.BrowserHeaders;
          }),
          /* 2 */
          /***/
          (function(r, o, i) {
            Object.defineProperty(o, "__esModule", { value: !0 });
            function n(s, m) {
              for (var u = s[Symbol.iterator](), l = u.next(); !l.done; )
                m(l.value[0]), l = u.next();
            }
            o.iterateHeaders = n;
            function f(s, m) {
              for (var u = s.keys(), l = u.next(); !l.done; )
                m(l.value), l = u.next();
            }
            o.iterateHeadersKeys = f;
          }),
          /* 3 */
          /***/
          (function(r, o, i) {
            Object.defineProperty(o, "__esModule", { value: !0 });
            var n = i(2);
            function f(d) {
              if (typeof d != "string" && (d = String(d)), /[^a-z0-9\-#$%&'*+.\^_`|~]/i.test(d))
                throw new TypeError("Invalid character in header field name");
              return d.toLowerCase();
            }
            o.normalizeName = f;
            function s(d) {
              return typeof d != "string" && (d = String(d)), d;
            }
            o.normalizeValue = s;
            function m(d, h) {
              var p = d;
              if (p instanceof Headers && p.getAll)
                return p.getAll(h);
              var v = p.get(h);
              return v && typeof v == "string" ? [v] : v;
            }
            o.getHeaderValues = m;
            function u(d) {
              return d;
            }
            function l(d) {
              var h = d, p = {}, v = [];
              return h.keys ? n.iterateHeadersKeys(h, function(a) {
                p[a] || (p[a] = !0, v.push(a));
              }) : h.forEach ? h.forEach(function(a, c) {
                p[c] || (p[c] = !0, v.push(c));
              }) : n.iterateHeaders(h, function(a) {
                var c = a[0];
                p[c] || (p[c] = !0, v.push(c));
              }), v;
            }
            o.getHeaderKeys = l;
            function b(d) {
              var h = [], p = d.split(", ");
              return p.forEach(function(v) {
                v.split(",").forEach(function(a) {
                  h.push(a);
                });
              }), h;
            }
            o.splitHeaderValue = b;
          })
          /******/
        ])
      );
    });
  })(pt)), pt.exports;
}
var kr = xr();
function Ee() {
  return {};
}
const mt = {
  encode(t, e = new S()) {
    return e;
  },
  decode(t, e) {
    const r = t instanceof k ? t : new k(t), o = e === void 0 ? r.len : r.pos + e, i = Ee();
    for (; r.pos < o; ) {
      const n = r.uint32();
      if ((n & 7) === 4 || n === 0)
        break;
      r.skip(n & 7);
    }
    return i;
  },
  fromJSON(t) {
    return {};
  },
  toJSON(t) {
    return {};
  },
  create(t) {
    return mt.fromPartial(t ?? {});
  },
  fromPartial(t) {
    return Ee();
  }
};
class gr {
  rpc;
  constructor(e) {
    this.rpc = e, this.EchoSwerveSample = this.EchoSwerveSample.bind(this), this.Generate = this.Generate.bind(this), this.GetDefaultTrajectory = this.GetDefaultTrajectory.bind(this);
  }
  EchoSwerveSample(e, r) {
    return this.rpc.unary(Le, vt.fromPartial(e), r);
  }
  Generate(e, r) {
    return this.rpc.unary(Fe, yt.fromPartial(e), r);
  }
  GetDefaultTrajectory(e, r) {
    return this.rpc.unary(Ue, mt.fromPartial(e), r);
  }
}
const bt = { serviceName: "service.ChoreoService" }, Le = {
  methodName: "EchoSwerveSample",
  service: bt,
  requestStream: !1,
  responseStream: !1,
  requestType: {
    serializeBinary() {
      return vt.encode(this).finish();
    }
  },
  responseType: {
    deserializeBinary(t) {
      const e = Et.decode(t);
      return {
        ...e,
        toObject() {
          return e;
        }
      };
    }
  }
}, Fe = {
  methodName: "Generate",
  service: bt,
  requestStream: !1,
  responseStream: !1,
  requestType: {
    serializeBinary() {
      return yt.encode(this).finish();
    }
  },
  responseType: {
    deserializeBinary(t) {
      const e = _t.decode(t);
      return {
        ...e,
        toObject() {
          return e;
        }
      };
    }
  }
}, Ue = {
  methodName: "GetDefaultTrajectory",
  service: bt,
  requestStream: !1,
  responseStream: !1,
  requestType: {
    serializeBinary() {
      return mt.encode(this).finish();
    }
  },
  responseType: {
    deserializeBinary(t) {
      const e = Pt.decode(t);
      return {
        ...e,
        toObject() {
          return e;
        }
      };
    }
  }
};
class Sr {
  host;
  options;
  constructor(e, r) {
    this.host = e, this.options = r;
  }
  unary(e, r, o) {
    const i = { ...r, ...e.requestType }, n = o && this.options.metadata ? new kr.BrowserHeaders({ ...this.options?.metadata.headersMap, ...o?.headersMap }) : o ?? this.options.metadata;
    return new Promise((f, s) => {
      Oe.grpc.unary(e, {
        request: i,
        host: this.host,
        metadata: n ?? {},
        ...this.options.transport !== void 0 ? { transport: this.options.transport } : {},
        debug: this.options.debug ?? !1,
        onEnd: function(m) {
          if (m.status === Oe.grpc.Code.OK)
            f(m.message.toObject());
          else {
            const u = new $e(m.statusMessage, m.status, m.trailers);
            s(u);
          }
        }
      });
    });
  }
}
class $e extends globalThis.Error {
  constructor(e, r, o) {
    super(e), this.code = r, this.metadata = o;
  }
}
const Or = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  ChoreoServiceClientImpl: gr,
  ChoreoServiceDesc: bt,
  ChoreoServiceEchoSwerveSampleDesc: Le,
  ChoreoServiceGenerateDesc: Fe,
  ChoreoServiceGetDefaultTrajectoryDesc: Ue,
  GrpcWebError: $e,
  GrpcWebImpl: Sr,
  commands: vr
}, Symbol.toStringTag, { value: "Module" })), Nr = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  Empty: mt
}, Symbol.toStringTag, { value: "Module" })), Tr = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  protobuf: Nr
}, Symbol.toStringTag, { value: "Module" }));
export {
  wr as entity,
  Tr as google,
  Or as service
};
//# sourceMappingURL=index.mjs.map
