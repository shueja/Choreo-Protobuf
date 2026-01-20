function we() {
  let t = 0, e = 0;
  for (let u = 0; u < 28; u += 7) {
    let i = this.buf[this.pos++];
    if (t |= (i & 127) << u, (i & 128) == 0)
      return this.assertBounds(), [t, e];
  }
  let r = this.buf[this.pos++];
  if (t |= (r & 15) << 28, e = (r & 112) >> 4, (r & 128) == 0)
    return this.assertBounds(), [t, e];
  for (let u = 3; u <= 31; u += 7) {
    let i = this.buf[this.pos++];
    if (e |= (i & 127) << u, (i & 128) == 0)
      return this.assertBounds(), [t, e];
  }
  throw new Error("invalid varint");
}
function ft(t, e, r) {
  for (let n = 0; n < 28; n = n + 7) {
    const s = t >>> n, a = !(!(s >>> 7) && e == 0), b = (a ? s | 128 : s) & 255;
    if (r.push(b), !a)
      return;
  }
  const u = t >>> 28 & 15 | (e & 7) << 4, i = e >> 3 != 0;
  if (r.push((i ? u | 128 : u) & 255), !!i) {
    for (let n = 3; n < 31; n = n + 7) {
      const s = e >>> n, a = !!(s >>> 7), b = (a ? s | 128 : s) & 255;
      if (r.push(b), !a)
        return;
    }
    r.push(e >>> 31 & 1);
  }
}
const tt = 4294967296;
function St(t) {
  const e = t[0] === "-";
  e && (t = t.slice(1));
  const r = 1e6;
  let u = 0, i = 0;
  function n(s, a) {
    const b = Number(t.slice(s, a));
    i *= r, u = u * r + b, u >= tt && (i = i + (u / tt | 0), u = u % tt);
  }
  return n(-24, -18), n(-18, -12), n(-12, -6), n(-6), e ? ue(u, i) : vt(u, i);
}
function Ne(t, e) {
  let r = vt(t, e);
  const u = r.hi & 2147483648;
  u && (r = ue(r.lo, r.hi));
  const i = se(r.lo, r.hi);
  return u ? "-" + i : i;
}
function se(t, e) {
  if ({ lo: t, hi: e } = Ee(t, e), e <= 2097151)
    return String(tt * e + t);
  const r = t & 16777215, u = (t >>> 24 | e << 8) & 16777215, i = e >> 16 & 65535;
  let n = r + u * 6777216 + i * 6710656, s = u + i * 8147497, a = i * 2;
  const b = 1e7;
  return n >= b && (s += Math.floor(n / b), n %= b), s >= b && (a += Math.floor(s / b), s %= b), a.toString() + gt(s) + gt(n);
}
function Ee(t, e) {
  return { lo: t >>> 0, hi: e >>> 0 };
}
function vt(t, e) {
  return { lo: t | 0, hi: e | 0 };
}
function ue(t, e) {
  return e = ~e, t ? t = ~t + 1 : e += 1, vt(t, e);
}
const gt = (t) => {
  const e = String(t);
  return "0000000".slice(e.length) + e;
};
function wt(t, e) {
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
function Oe() {
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
const C = /* @__PURE__ */ _e();
function _e() {
  const t = new DataView(new ArrayBuffer(8));
  if (typeof BigInt == "function" && typeof t.getBigInt64 == "function" && typeof t.getBigUint64 == "function" && typeof t.setBigInt64 == "function" && typeof t.setBigUint64 == "function" && (!!globalThis.Deno || typeof process != "object" || typeof process.env != "object" || process.env.BUF_BIGINT_DISABLE !== "1")) {
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
        return t.setBigInt64(0, this.parse(s), !0), {
          lo: t.getInt32(0, !0),
          hi: t.getInt32(4, !0)
        };
      },
      uEnc(s) {
        return t.setBigInt64(0, this.uParse(s), !0), {
          lo: t.getInt32(0, !0),
          hi: t.getInt32(4, !0)
        };
      },
      dec(s, a) {
        return t.setInt32(0, s, !0), t.setInt32(4, a, !0), t.getBigInt64(0, !0);
      },
      uDec(s, a) {
        return t.setInt32(0, s, !0), t.setInt32(4, a, !0), t.getBigUint64(0, !0);
      }
    };
  }
  return {
    zero: "0",
    supported: !1,
    parse(r) {
      return typeof r != "string" && (r = r.toString()), Nt(r), r;
    },
    uParse(r) {
      return typeof r != "string" && (r = r.toString()), Et(r), r;
    },
    enc(r) {
      return typeof r != "string" && (r = r.toString()), Nt(r), St(r);
    },
    uEnc(r) {
      return typeof r != "string" && (r = r.toString()), Et(r), St(r);
    },
    dec(r, u) {
      return Ne(r, u);
    },
    uDec(r, u) {
      return se(r, u);
    }
  };
}
function Nt(t) {
  if (!/^-?[0-9]+$/.test(t))
    throw new Error("invalid int64: " + t);
}
function Et(t) {
  if (!/^[0-9]+$/.test(t))
    throw new Error("invalid uint64: " + t);
}
const ct = /* @__PURE__ */ Symbol.for("@bufbuild/protobuf/text-encoding");
function le() {
  if (globalThis[ct] == null) {
    const t = new globalThis.TextEncoder(), e = new globalThis.TextDecoder();
    globalThis[ct] = {
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
  return globalThis[ct];
}
var B;
(function(t) {
  t[t.Varint = 0] = "Varint", t[t.Bit64 = 1] = "Bit64", t[t.LengthDelimited = 2] = "LengthDelimited", t[t.StartGroup = 3] = "StartGroup", t[t.EndGroup = 4] = "EndGroup", t[t.Bit32 = 5] = "Bit32";
})(B || (B = {}));
const Te = 34028234663852886e22, Pe = -34028234663852886e22, Me = 4294967295, Je = 2147483647, Re = -2147483648;
class w {
  constructor(e = le().encodeUtf8) {
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
    let r = new Uint8Array(e), u = 0;
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
    for (Ot(e); e > 127; )
      this.buf.push(e & 127 | 128), e = e >>> 7;
    return this.buf.push(e), this;
  }
  /**
   * Write a `int32` value, a signed 32 bit varint.
   */
  int32(e) {
    return dt(e), wt(e, this.buf), this;
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
    Ie(e);
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
    Ot(e);
    let r = new Uint8Array(4);
    return new DataView(r.buffer).setUint32(0, e, !0), this.raw(r);
  }
  /**
   * Write a `sfixed32` value, a signed, fixed-length 32-bit integer.
   */
  sfixed32(e) {
    dt(e);
    let r = new Uint8Array(4);
    return new DataView(r.buffer).setInt32(0, e, !0), this.raw(r);
  }
  /**
   * Write a `sint32` value, a signed, zigzag-encoded 32-bit varint.
   */
  sint32(e) {
    return dt(e), e = (e << 1 ^ e >> 31) >>> 0, wt(e, this.buf), this;
  }
  /**
   * Write a `fixed64` value, a signed, fixed-length 64-bit integer.
   */
  sfixed64(e) {
    let r = new Uint8Array(8), u = new DataView(r.buffer), i = C.enc(e);
    return u.setInt32(0, i.lo, !0), u.setInt32(4, i.hi, !0), this.raw(r);
  }
  /**
   * Write a `fixed64` value, an unsigned, fixed-length 64 bit integer.
   */
  fixed64(e) {
    let r = new Uint8Array(8), u = new DataView(r.buffer), i = C.uEnc(e);
    return u.setInt32(0, i.lo, !0), u.setInt32(4, i.hi, !0), this.raw(r);
  }
  /**
   * Write a `int64` value, a signed 64-bit varint.
   */
  int64(e) {
    let r = C.enc(e);
    return ft(r.lo, r.hi, this.buf), this;
  }
  /**
   * Write a `sint64` value, a signed, zig-zag-encoded 64-bit varint.
   */
  sint64(e) {
    const r = C.enc(e), u = r.hi >> 31, i = r.lo << 1 ^ u, n = (r.hi << 1 | r.lo >>> 31) ^ u;
    return ft(i, n, this.buf), this;
  }
  /**
   * Write a `uint64` value, an unsigned 64-bit varint.
   */
  uint64(e) {
    const r = C.uEnc(e);
    return ft(r.lo, r.hi, this.buf), this;
  }
}
class S {
  constructor(e, r = le().decodeUtf8) {
    this.decodeUtf8 = r, this.varint64 = we, this.uint32 = Oe, this.buf = e, this.len = e.length, this.pos = 0, this.view = new DataView(e.buffer, e.byteOffset, e.byteLength);
  }
  /**
   * Reads a tag - field number and wire type.
   */
  tag() {
    let e = this.uint32(), r = e >>> 3, u = e & 7;
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
  skip(e, r) {
    let u = this.pos;
    switch (e) {
      case B.Varint:
        for (; this.buf[this.pos++] & 128; )
          ;
        break;
      // @ts-ignore TS7029: Fallthrough case in switch -- ignore instead of expect-error for compiler settings without noFallthroughCasesInSwitch: true
      case B.Bit64:
        this.pos += 4;
      case B.Bit32:
        this.pos += 4;
        break;
      case B.LengthDelimited:
        let i = this.uint32();
        this.pos += i;
        break;
      case B.StartGroup:
        for (; ; ) {
          const [n, s] = this.tag();
          if (s === B.EndGroup) {
            if (r !== void 0 && n !== r)
              throw new Error("invalid end group tag");
            break;
          }
          this.skip(s, n);
        }
        break;
      default:
        throw new Error("cant skip wire type " + e);
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
    let e = this.uint32();
    return e >>> 1 ^ -(e & 1);
  }
  /**
   * Read a `int64` field, a signed 64-bit varint.
   */
  int64() {
    return C.dec(...this.varint64());
  }
  /**
   * Read a `uint64` field, an unsigned 64-bit varint.
   */
  uint64() {
    return C.uDec(...this.varint64());
  }
  /**
   * Read a `sint64` field, a signed, zig-zag-encoded 64-bit varint.
   */
  sint64() {
    let [e, r] = this.varint64(), u = -(e & 1);
    return e = (e >>> 1 | (r & 1) << 31) ^ u, r = r >>> 1 ^ u, C.dec(e, r);
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
    return C.uDec(this.sfixed32(), this.sfixed32());
  }
  /**
   * Read a `fixed64` field, a signed, fixed-length 64-bit integer.
   */
  sfixed64() {
    return C.dec(this.sfixed32(), this.sfixed32());
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
function dt(t) {
  if (typeof t == "string")
    t = Number(t);
  else if (typeof t != "number")
    throw new Error("invalid int32: " + typeof t);
  if (!Number.isInteger(t) || t > Je || t < Re)
    throw new Error("invalid int32: " + t);
}
function Ot(t) {
  if (typeof t == "string")
    t = Number(t);
  else if (typeof t != "number")
    throw new Error("invalid uint32: " + typeof t);
  if (!Number.isInteger(t) || t > Me || t < 0)
    throw new Error("invalid uint32: " + t);
}
function Ie(t) {
  if (typeof t == "string") {
    const e = t;
    if (t = Number(t), Number.isNaN(t) && e !== "NaN")
      throw new Error("invalid float32: " + e);
  } else if (typeof t != "number")
    throw new Error("invalid float32: " + typeof t);
  if (Number.isFinite(t) && (t > Te || t < Pe))
    throw new Error("invalid float32: " + t);
}
function _t() {
  return { t: 0, x: 0, y: 0, heading: 0, vl: 0, vr: 0, omega: 0, al: 0, ar: 0, alpha: 0, fl: 0, fr: 0 };
}
const U = {
  encode(t, e = new w()) {
    return t.t !== 0 && e.uint32(9).double(t.t), t.x !== 0 && e.uint32(17).double(t.x), t.y !== 0 && e.uint32(25).double(t.y), t.heading !== 0 && e.uint32(33).double(t.heading), t.vl !== 0 && e.uint32(41).double(t.vl), t.vr !== 0 && e.uint32(49).double(t.vr), t.omega !== 0 && e.uint32(57).double(t.omega), t.al !== 0 && e.uint32(65).double(t.al), t.ar !== 0 && e.uint32(73).double(t.ar), t.alpha !== 0 && e.uint32(81).double(t.alpha), t.fl !== 0 && e.uint32(89).double(t.fl), t.fr !== 0 && e.uint32(97).double(t.fr), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = _t();
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
  fromJSON(t) {
    return {
      t: J(t.t) ? globalThis.Number(t.t) : 0,
      x: J(t.x) ? globalThis.Number(t.x) : 0,
      y: J(t.y) ? globalThis.Number(t.y) : 0,
      heading: J(t.heading) ? globalThis.Number(t.heading) : 0,
      vl: J(t.vl) ? globalThis.Number(t.vl) : 0,
      vr: J(t.vr) ? globalThis.Number(t.vr) : 0,
      omega: J(t.omega) ? globalThis.Number(t.omega) : 0,
      al: J(t.al) ? globalThis.Number(t.al) : 0,
      ar: J(t.ar) ? globalThis.Number(t.ar) : 0,
      alpha: J(t.alpha) ? globalThis.Number(t.alpha) : 0,
      fl: J(t.fl) ? globalThis.Number(t.fl) : 0,
      fr: J(t.fr) ? globalThis.Number(t.fr) : 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.t !== 0 && (e.t = t.t), t.x !== 0 && (e.x = t.x), t.y !== 0 && (e.y = t.y), t.heading !== 0 && (e.heading = t.heading), t.vl !== 0 && (e.vl = t.vl), t.vr !== 0 && (e.vr = t.vr), t.omega !== 0 && (e.omega = t.omega), t.al !== 0 && (e.al = t.al), t.ar !== 0 && (e.ar = t.ar), t.alpha !== 0 && (e.alpha = t.alpha), t.fl !== 0 && (e.fl = t.fl), t.fr !== 0 && (e.fr = t.fr), e;
  },
  create(t) {
    return U.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = _t();
    return e.t = t.t ?? 0, e.x = t.x ?? 0, e.y = t.y ?? 0, e.heading = t.heading ?? 0, e.vl = t.vl ?? 0, e.vr = t.vr ?? 0, e.omega = t.omega ?? 0, e.al = t.al ?? 0, e.ar = t.ar ?? 0, e.alpha = t.alpha ?? 0, e.fl = t.fl ?? 0, e.fr = t.fr ?? 0, e;
  }
};
function J(t) {
  return t != null;
}
var fe = /* @__PURE__ */ ((t) => (t[t.DRIVE_TYPE_SWERVE = 0] = "DRIVE_TYPE_SWERVE", t[t.DRIVE_TYPE_DIFFERENTIAL = 1] = "DRIVE_TYPE_DIFFERENTIAL", t[t.DRIVE_TYPE_MECANUM = 2] = "DRIVE_TYPE_MECANUM", t[t.UNRECOGNIZED = -1] = "UNRECOGNIZED", t))(fe || {});
function pt(t) {
  switch (t) {
    case 0:
    case "DRIVE_TYPE_SWERVE":
      return 0;
    case 1:
    case "DRIVE_TYPE_DIFFERENTIAL":
      return 1;
    case 2:
    case "DRIVE_TYPE_MECANUM":
      return 2;
    default:
      return -1;
  }
}
function ce(t) {
  switch (t) {
    case 0:
      return "DRIVE_TYPE_SWERVE";
    case 1:
      return "DRIVE_TYPE_DIFFERENTIAL";
    case 2:
      return "DRIVE_TYPE_MECANUM";
    default:
      return "UNRECOGNIZED";
  }
}
function Tt() {
  return { value: 0, expr: "" };
}
const y = {
  encode(t, e = new w()) {
    return t.value !== 0 && e.uint32(9).double(t.value), t.expr !== "" && e.uint32(18).string(t.expr), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Tt();
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
  fromJSON(t) {
    return {
      value: Pt(t.value) ? globalThis.Number(t.value) : 0,
      expr: Pt(t.expr) ? globalThis.String(t.expr) : ""
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
    const e = Tt();
    return e.value = t.value ?? 0, e.expr = t.expr ?? "", e;
  }
};
function Pt(t) {
  return t != null;
}
function Mt() {
  return { max: null };
}
const D = {
  encode(t, e = new w()) {
    return t.max !== void 0 && t.max !== null && y.encode(t.max, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Mt();
    for (; r.pos < u; ) {
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
    return { max: he(t.max) ? y.fromJSON(t.max) : null };
  },
  toJSON(t) {
    const e = {};
    return t.max !== void 0 && t.max !== null && (e.max = y.toJSON(t.max)), e;
  },
  create(t) {
    return D.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Mt();
    return e.max = t.max !== void 0 && t.max !== null ? y.fromPartial(t.max) : null, e;
  }
};
function Jt() {
  return { test: "" };
}
const de = {
  encode(t, e = new w()) {
    return t.test !== "" && e.uint32(10).string(t.test), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Jt();
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
  fromJSON(t) {
    return { test: he(t.test) ? globalThis.String(t.test) : "" };
  },
  toJSON(t) {
    const e = {};
    return t.test !== "" && (e.test = t.test), e;
  },
  create(t) {
    return de.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Jt();
    return e.test = t.test ?? "", e;
  }
};
function he(t) {
  return t != null;
}
const Ae = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  ExprMaxVelocity: D,
  TestExpr: de
}, Symbol.toStringTag, { value: "Module" }));
function Rt() {
  return { max: null };
}
const F = {
  encode(t, e = new w()) {
    return t.max !== void 0 && t.max !== null && y.encode(t.max, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Rt();
    for (; r.pos < u; ) {
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
    return { max: He(t.max) ? y.fromJSON(t.max) : null };
  },
  toJSON(t) {
    const e = {};
    return t.max !== void 0 && t.max !== null && (e.max = y.toJSON(t.max)), e;
  },
  create(t) {
    return F.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Rt();
    return e.max = t.max !== void 0 && t.max !== null ? y.fromPartial(t.max) : null, e;
  }
};
function He(t) {
  return t != null;
}
const Ce = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  ExprMaxAcceleration: F
}, Symbol.toStringTag, { value: "Module" }));
function It() {
  return {};
}
const z = {
  encode(t, e = new w()) {
    return e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = It();
    for (; r.pos < u; ) {
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
    return z.fromPartial(t ?? {});
  },
  fromPartial(t) {
    return It();
  }
};
function At() {
  return {};
}
const G = {
  encode(t, e = new w()) {
    return e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = At();
    for (; r.pos < u; ) {
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
    return G.fromPartial(t ?? {});
  },
  fromPartial(t) {
    return At();
  }
};
function Ht() {
  return { idx: 0 };
}
const $ = {
  encode(t, e = new w()) {
    return t.idx !== 0 && e.uint32(8).uint64(t.idx), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Ht();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 8)
            break;
          i.idx = Be(r.uint64());
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
    return { idx: et(t.idx) ? globalThis.Number(t.idx) : 0 };
  },
  toJSON(t) {
    const e = {};
    return t.idx !== 0 && (e.idx = Math.round(t.idx)), e;
  },
  create(t) {
    return $.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Ht();
    return e.idx = t.idx ?? 0, e;
  }
};
function Ct() {
  return { id: null };
}
const R = {
  encode(t, e = new w()) {
    switch (t.id?.$case) {
      case "first":
        z.encode(t.id.value, e.uint32(10).fork()).join();
        break;
      case "last":
        G.encode(t.id.value, e.uint32(18).fork()).join();
        break;
      case "idx":
        $.encode(t.id.value, e.uint32(26).fork()).join();
        break;
    }
    return e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Ct();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.id = { $case: "first", value: z.decode(r, r.uint32()) };
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.id = { $case: "last", value: G.decode(r, r.uint32()) };
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.id = { $case: "idx", value: $.decode(r, r.uint32()) };
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
      id: et(t.first) ? { $case: "first", value: z.fromJSON(t.first) } : et(t.last) ? { $case: "last", value: G.fromJSON(t.last) } : et(t.idx) ? { $case: "idx", value: $.fromJSON(t.idx) } : null
    };
  },
  toJSON(t) {
    const e = {};
    return t.id?.$case === "first" ? e.first = z.toJSON(t.id.value) : t.id?.$case === "last" ? e.last = G.toJSON(t.id.value) : t.id?.$case === "idx" && (e.idx = $.toJSON(t.id.value)), e;
  },
  create(t) {
    return R.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Ct();
    switch (t.id?.$case) {
      case "first": {
        t.id?.value !== void 0 && t.id?.value !== null && (e.id = { $case: "first", value: z.fromPartial(t.id.value) });
        break;
      }
      case "last": {
        t.id?.value !== void 0 && t.id?.value !== null && (e.id = { $case: "last", value: G.fromPartial(t.id.value) });
        break;
      }
      case "idx": {
        t.id?.value !== void 0 && t.id?.value !== null && (e.id = { $case: "idx", value: $.fromPartial(t.id.value) });
        break;
      }
    }
    return e;
  }
};
function Be(t) {
  const e = globalThis.Number(t.toString());
  if (e > globalThis.Number.MAX_SAFE_INTEGER)
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  if (e < globalThis.Number.MIN_SAFE_INTEGER)
    throw new globalThis.Error("Value is smaller than Number.MIN_SAFE_INTEGER");
  return e;
}
function et(t) {
  return t != null;
}
function Bt() {
  return { enabled: !1, from: null, to: null, data: null };
}
const X = {
  encode(t, e = new w()) {
    switch (t.enabled !== !1 && e.uint32(8).bool(t.enabled), t.from !== void 0 && t.from !== null && R.encode(t.from, e.uint32(18).fork()).join(), t.to !== void 0 && t.to !== null && R.encode(t.to, e.uint32(26).fork()).join(), t.data?.$case) {
      case "maxVelocity":
        D.encode(t.data.value, e.uint32(34).fork()).join();
        break;
      case "maxAcceleration":
        F.encode(t.data.value, e.uint32(42).fork()).join();
        break;
    }
    return e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Bt();
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
          i.from = R.decode(r, r.uint32());
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.to = R.decode(r, r.uint32());
          continue;
        }
        case 4: {
          if (n !== 34)
            break;
          i.data = { $case: "maxVelocity", value: D.decode(r, r.uint32()) };
          continue;
        }
        case 5: {
          if (n !== 42)
            break;
          i.data = { $case: "maxAcceleration", value: F.decode(r, r.uint32()) };
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
      enabled: V(t.enabled) ? globalThis.Boolean(t.enabled) : !1,
      from: V(t.from) ? R.fromJSON(t.from) : null,
      to: V(t.to) ? R.fromJSON(t.to) : null,
      data: V(t.maxVelocity) ? { $case: "maxVelocity", value: D.fromJSON(t.maxVelocity) } : V(t.max_velocity) ? { $case: "maxVelocity", value: D.fromJSON(t.max_velocity) } : V(t.maxAcceleration) ? { $case: "maxAcceleration", value: F.fromJSON(t.maxAcceleration) } : V(t.max_acceleration) ? { $case: "maxAcceleration", value: F.fromJSON(t.max_acceleration) } : null
    };
  },
  toJSON(t) {
    const e = {};
    return t.enabled !== !1 && (e.enabled = t.enabled), t.from !== void 0 && t.from !== null && (e.from = R.toJSON(t.from)), t.to !== void 0 && t.to !== null && (e.to = R.toJSON(t.to)), t.data?.$case === "maxVelocity" ? e.maxVelocity = D.toJSON(t.data.value) : t.data?.$case === "maxAcceleration" && (e.maxAcceleration = F.toJSON(t.data.value)), e;
  },
  create(t) {
    return X.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Bt();
    switch (e.enabled = t.enabled ?? !1, e.from = t.from !== void 0 && t.from !== null ? R.fromPartial(t.from) : null, e.to = t.to !== void 0 && t.to !== null ? R.fromPartial(t.to) : null, t.data?.$case) {
      case "maxVelocity": {
        t.data?.value !== void 0 && t.data?.value !== null && (e.data = { $case: "maxVelocity", value: D.fromPartial(t.data.value) });
        break;
      }
      case "maxAcceleration": {
        t.data?.value !== void 0 && t.data?.value !== null && (e.data = { $case: "maxAcceleration", value: F.fromPartial(t.data.value) });
        break;
      }
    }
    return e;
  }
};
function V(t) {
  return t != null;
}
const De = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  ExprConstraint: X,
  max_acceleration: Ce,
  maxvelocity: Ae
}, Symbol.toStringTag, { value: "Module" }));
function Dt() {
  return {
    x: null,
    y: null,
    heading: null,
    intervals: 0,
    split: !1,
    fixTranslation: !1,
    fixHeading: !1,
    overrideIntervals: !1
  };
}
const q = {
  encode(t, e = new w()) {
    return t.x !== void 0 && t.x !== null && y.encode(t.x, e.uint32(10).fork()).join(), t.y !== void 0 && t.y !== null && y.encode(t.y, e.uint32(18).fork()).join(), t.heading !== void 0 && t.heading !== null && y.encode(t.heading, e.uint32(26).fork()).join(), t.intervals !== 0 && e.uint32(32).uint64(t.intervals), t.split !== !1 && e.uint32(40).bool(t.split), t.fixTranslation !== !1 && e.uint32(48).bool(t.fixTranslation), t.fixHeading !== !1 && e.uint32(56).bool(t.fixHeading), t.overrideIntervals !== !1 && e.uint32(64).bool(t.overrideIntervals), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Dt();
    for (; r.pos < u; ) {
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
          i.intervals = Fe(r.uint64());
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
      x: H(t.x) ? y.fromJSON(t.x) : null,
      y: H(t.y) ? y.fromJSON(t.y) : null,
      heading: H(t.heading) ? y.fromJSON(t.heading) : null,
      intervals: H(t.intervals) ? globalThis.Number(t.intervals) : 0,
      split: H(t.split) ? globalThis.Boolean(t.split) : !1,
      fixTranslation: H(t.fixTranslation) ? globalThis.Boolean(t.fixTranslation) : H(t.fix_translation) ? globalThis.Boolean(t.fix_translation) : !1,
      fixHeading: H(t.fixHeading) ? globalThis.Boolean(t.fixHeading) : H(t.fix_heading) ? globalThis.Boolean(t.fix_heading) : !1,
      overrideIntervals: H(t.overrideIntervals) ? globalThis.Boolean(t.overrideIntervals) : H(t.override_intervals) ? globalThis.Boolean(t.override_intervals) : !1
    };
  },
  toJSON(t) {
    const e = {};
    return t.x !== void 0 && t.x !== null && (e.x = y.toJSON(t.x)), t.y !== void 0 && t.y !== null && (e.y = y.toJSON(t.y)), t.heading !== void 0 && t.heading !== null && (e.heading = y.toJSON(t.heading)), t.intervals !== 0 && (e.intervals = Math.round(t.intervals)), t.split !== !1 && (e.split = t.split), t.fixTranslation !== !1 && (e.fixTranslation = t.fixTranslation), t.fixHeading !== !1 && (e.fixHeading = t.fixHeading), t.overrideIntervals !== !1 && (e.overrideIntervals = t.overrideIntervals), e;
  },
  create(t) {
    return q.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Dt();
    return e.x = t.x !== void 0 && t.x !== null ? y.fromPartial(t.x) : null, e.y = t.y !== void 0 && t.y !== null ? y.fromPartial(t.y) : null, e.heading = t.heading !== void 0 && t.heading !== null ? y.fromPartial(t.heading) : null, e.intervals = t.intervals ?? 0, e.split = t.split ?? !1, e.fixTranslation = t.fixTranslation ?? !1, e.fixHeading = t.fixHeading ?? !1, e.overrideIntervals = t.overrideIntervals ?? !1, e;
  }
};
function Fe(t) {
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
const Le = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  ExprWaypoint: q
}, Symbol.toStringTag, { value: "Module" }));
function Ft() {
  return { targetDt: null, waypoints: [], constraints: [] };
}
const I = {
  encode(t, e = new w()) {
    t.targetDt !== void 0 && t.targetDt !== null && y.encode(t.targetDt, e.uint32(10).fork()).join();
    for (const r of t.waypoints)
      q.encode(r, e.uint32(18).fork()).join();
    for (const r of t.constraints)
      X.encode(r, e.uint32(26).fork()).join();
    return e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Ft();
    for (; r.pos < u; ) {
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
          i.waypoints.push(q.decode(r, r.uint32()));
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.constraints.push(X.decode(r, r.uint32()));
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
      targetDt: Lt(t.targetDt) ? y.fromJSON(t.targetDt) : Lt(t.target_dt) ? y.fromJSON(t.target_dt) : null,
      waypoints: globalThis.Array.isArray(t?.waypoints) ? t.waypoints.map((e) => q.fromJSON(e)) : [],
      constraints: globalThis.Array.isArray(t?.constraints) ? t.constraints.map((e) => X.fromJSON(e)) : []
    };
  },
  toJSON(t) {
    const e = {};
    return t.targetDt !== void 0 && t.targetDt !== null && (e.targetDt = y.toJSON(t.targetDt)), t.waypoints?.length && (e.waypoints = t.waypoints.map((r) => q.toJSON(r))), t.constraints?.length && (e.constraints = t.constraints.map((r) => X.toJSON(r))), e;
  },
  create(t) {
    return I.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Ft();
    return e.targetDt = t.targetDt !== void 0 && t.targetDt !== null ? y.fromPartial(t.targetDt) : null, e.waypoints = t.waypoints?.map((r) => q.fromPartial(r)) || [], e.constraints = t.constraints?.map((r) => X.fromPartial(r)) || [], e;
  }
};
function Lt(t) {
  return t != null;
}
function Vt() {
  return { x: null, y: null };
}
const N = {
  encode(t, e = new w()) {
    return t.x !== void 0 && t.x !== null && y.encode(t.x, e.uint32(10).fork()).join(), t.y !== void 0 && t.y !== null && y.encode(t.y, e.uint32(18).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Vt();
    for (; r.pos < u; ) {
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
    return { x: E(t.x) ? y.fromJSON(t.x) : null, y: E(t.y) ? y.fromJSON(t.y) : null };
  },
  toJSON(t) {
    const e = {};
    return t.x !== void 0 && t.x !== null && (e.x = y.toJSON(t.x)), t.y !== void 0 && t.y !== null && (e.y = y.toJSON(t.y)), e;
  },
  create(t) {
    return N.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Vt();
    return e.x = t.x !== void 0 && t.x !== null ? y.fromPartial(t.x) : null, e.y = t.y !== void 0 && t.y !== null ? y.fromPartial(t.y) : null, e;
  }
};
function Ut() {
  return { front: null, left: null, right: null, back: null };
}
const W = {
  encode(t, e = new w()) {
    return t.front !== void 0 && t.front !== null && y.encode(t.front, e.uint32(10).fork()).join(), t.left !== void 0 && t.left !== null && y.encode(t.left, e.uint32(18).fork()).join(), t.right !== void 0 && t.right !== null && y.encode(t.right, e.uint32(26).fork()).join(), t.back !== void 0 && t.back !== null && y.encode(t.back, e.uint32(34).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Ut();
    for (; r.pos < u; ) {
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
      front: E(t.front) ? y.fromJSON(t.front) : null,
      left: E(t.left) ? y.fromJSON(t.left) : null,
      right: E(t.right) ? y.fromJSON(t.right) : null,
      back: E(t.back) ? y.fromJSON(t.back) : null
    };
  },
  toJSON(t) {
    const e = {};
    return t.front !== void 0 && t.front !== null && (e.front = y.toJSON(t.front)), t.left !== void 0 && t.left !== null && (e.left = y.toJSON(t.left)), t.right !== void 0 && t.right !== null && (e.right = y.toJSON(t.right)), t.back !== void 0 && t.back !== null && (e.back = y.toJSON(t.back)), e;
  },
  create(t) {
    return W.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Ut();
    return e.front = t.front !== void 0 && t.front !== null ? y.fromPartial(t.front) : null, e.left = t.left !== void 0 && t.left !== null ? y.fromPartial(t.left) : null, e.right = t.right !== void 0 && t.right !== null ? y.fromPartial(t.right) : null, e.back = t.back !== void 0 && t.back !== null ? y.fromPartial(t.back) : null, e;
  }
};
function zt() {
  return {
    mass: null,
    inertia: null,
    gearing: null,
    radius: null,
    vmax: null,
    tmax: null,
    cof: null,
    differentialTrackWidth: null,
    bumper: null,
    frontLeft: null,
    frontRight: null,
    backLeft: null,
    backRight: null
  };
}
const A = {
  encode(t, e = new w()) {
    return t.mass !== void 0 && t.mass !== null && y.encode(t.mass, e.uint32(10).fork()).join(), t.inertia !== void 0 && t.inertia !== null && y.encode(t.inertia, e.uint32(18).fork()).join(), t.gearing !== void 0 && t.gearing !== null && y.encode(t.gearing, e.uint32(26).fork()).join(), t.radius !== void 0 && t.radius !== null && y.encode(t.radius, e.uint32(34).fork()).join(), t.vmax !== void 0 && t.vmax !== null && y.encode(t.vmax, e.uint32(42).fork()).join(), t.tmax !== void 0 && t.tmax !== null && y.encode(t.tmax, e.uint32(50).fork()).join(), t.cof !== void 0 && t.cof !== null && y.encode(t.cof, e.uint32(58).fork()).join(), t.differentialTrackWidth !== void 0 && t.differentialTrackWidth !== null && y.encode(t.differentialTrackWidth, e.uint32(66).fork()).join(), t.bumper !== void 0 && t.bumper !== null && W.encode(t.bumper, e.uint32(74).fork()).join(), t.frontLeft !== void 0 && t.frontLeft !== null && N.encode(t.frontLeft, e.uint32(82).fork()).join(), t.frontRight !== void 0 && t.frontRight !== null && N.encode(t.frontRight, e.uint32(90).fork()).join(), t.backLeft !== void 0 && t.backLeft !== null && N.encode(t.backLeft, e.uint32(98).fork()).join(), t.backRight !== void 0 && t.backRight !== null && N.encode(t.backRight, e.uint32(106).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = zt();
    for (; r.pos < u; ) {
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
          i.bumper = W.decode(r, r.uint32());
          continue;
        }
        case 10: {
          if (n !== 82)
            break;
          i.frontLeft = N.decode(r, r.uint32());
          continue;
        }
        case 11: {
          if (n !== 90)
            break;
          i.frontRight = N.decode(r, r.uint32());
          continue;
        }
        case 12: {
          if (n !== 98)
            break;
          i.backLeft = N.decode(r, r.uint32());
          continue;
        }
        case 13: {
          if (n !== 106)
            break;
          i.backRight = N.decode(r, r.uint32());
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
      mass: E(t.mass) ? y.fromJSON(t.mass) : null,
      inertia: E(t.inertia) ? y.fromJSON(t.inertia) : null,
      gearing: E(t.gearing) ? y.fromJSON(t.gearing) : null,
      radius: E(t.radius) ? y.fromJSON(t.radius) : null,
      vmax: E(t.vmax) ? y.fromJSON(t.vmax) : null,
      tmax: E(t.tmax) ? y.fromJSON(t.tmax) : null,
      cof: E(t.cof) ? y.fromJSON(t.cof) : null,
      differentialTrackWidth: E(t.differentialTrackWidth) ? y.fromJSON(t.differentialTrackWidth) : E(t.differential_track_width) ? y.fromJSON(t.differential_track_width) : null,
      bumper: E(t.bumper) ? W.fromJSON(t.bumper) : null,
      frontLeft: E(t.frontLeft) ? N.fromJSON(t.frontLeft) : E(t.front_left) ? N.fromJSON(t.front_left) : null,
      frontRight: E(t.frontRight) ? N.fromJSON(t.frontRight) : E(t.front_right) ? N.fromJSON(t.front_right) : null,
      backLeft: E(t.backLeft) ? N.fromJSON(t.backLeft) : E(t.back_left) ? N.fromJSON(t.back_left) : null,
      backRight: E(t.backRight) ? N.fromJSON(t.backRight) : E(t.back_right) ? N.fromJSON(t.back_right) : null
    };
  },
  toJSON(t) {
    const e = {};
    return t.mass !== void 0 && t.mass !== null && (e.mass = y.toJSON(t.mass)), t.inertia !== void 0 && t.inertia !== null && (e.inertia = y.toJSON(t.inertia)), t.gearing !== void 0 && t.gearing !== null && (e.gearing = y.toJSON(t.gearing)), t.radius !== void 0 && t.radius !== null && (e.radius = y.toJSON(t.radius)), t.vmax !== void 0 && t.vmax !== null && (e.vmax = y.toJSON(t.vmax)), t.tmax !== void 0 && t.tmax !== null && (e.tmax = y.toJSON(t.tmax)), t.cof !== void 0 && t.cof !== null && (e.cof = y.toJSON(t.cof)), t.differentialTrackWidth !== void 0 && t.differentialTrackWidth !== null && (e.differentialTrackWidth = y.toJSON(t.differentialTrackWidth)), t.bumper !== void 0 && t.bumper !== null && (e.bumper = W.toJSON(t.bumper)), t.frontLeft !== void 0 && t.frontLeft !== null && (e.frontLeft = N.toJSON(t.frontLeft)), t.frontRight !== void 0 && t.frontRight !== null && (e.frontRight = N.toJSON(t.frontRight)), t.backLeft !== void 0 && t.backLeft !== null && (e.backLeft = N.toJSON(t.backLeft)), t.backRight !== void 0 && t.backRight !== null && (e.backRight = N.toJSON(t.backRight)), e;
  },
  create(t) {
    return A.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = zt();
    return e.mass = t.mass !== void 0 && t.mass !== null ? y.fromPartial(t.mass) : null, e.inertia = t.inertia !== void 0 && t.inertia !== null ? y.fromPartial(t.inertia) : null, e.gearing = t.gearing !== void 0 && t.gearing !== null ? y.fromPartial(t.gearing) : null, e.radius = t.radius !== void 0 && t.radius !== null ? y.fromPartial(t.radius) : null, e.vmax = t.vmax !== void 0 && t.vmax !== null ? y.fromPartial(t.vmax) : null, e.tmax = t.tmax !== void 0 && t.tmax !== null ? y.fromPartial(t.tmax) : null, e.cof = t.cof !== void 0 && t.cof !== null ? y.fromPartial(t.cof) : null, e.differentialTrackWidth = t.differentialTrackWidth !== void 0 && t.differentialTrackWidth !== null ? y.fromPartial(t.differentialTrackWidth) : null, e.bumper = t.bumper !== void 0 && t.bumper !== null ? W.fromPartial(t.bumper) : null, e.frontLeft = t.frontLeft !== void 0 && t.frontLeft !== null ? N.fromPartial(t.frontLeft) : null, e.frontRight = t.frontRight !== void 0 && t.frontRight !== null ? N.fromPartial(t.frontRight) : null, e.backLeft = t.backLeft !== void 0 && t.backLeft !== null ? N.fromPartial(t.backLeft) : null, e.backRight = t.backRight !== void 0 && t.backRight !== null ? N.fromPartial(t.backRight) : null, e;
  }
};
function E(t) {
  return t != null;
}
const Ve = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  ExprBumper: W,
  ExprModule: N,
  ExprRobotConfig: A
}, Symbol.toStringTag, { value: "Module" })), Ue = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  ExprParameters: I,
  WaypointID: R,
  WaypointIDFirst: z,
  WaypointIDLast: G,
  WaypointIDX: $,
  constraint: De,
  robotconfig: Ve,
  waypoint: Le
}, Symbol.toStringTag, { value: "Module" }));
function Gt() {
  return { name: "", config: null, driveType: 0 };
}
const pe = {
  encode(t, e = new w()) {
    return t.name !== "" && e.uint32(10).string(t.name), t.config !== void 0 && t.config !== null && A.encode(t.config, e.uint32(18).fork()).join(), t.driveType !== 0 && e.uint32(24).int32(t.driveType), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Gt();
    for (; r.pos < u; ) {
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
          i.config = A.decode(r, r.uint32());
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
      name: Q(t.name) ? globalThis.String(t.name) : "",
      config: Q(t.config) ? A.fromJSON(t.config) : null,
      driveType: Q(t.driveType) ? pt(t.driveType) : Q(t.drive_type) ? pt(t.drive_type) : 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.name !== "" && (e.name = t.name), t.config !== void 0 && t.config !== null && (e.config = A.toJSON(t.config)), t.driveType !== 0 && (e.driveType = ce(t.driveType)), e;
  },
  create(t) {
    return pe.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Gt();
    return e.name = t.name ?? "", e.config = t.config !== void 0 && t.config !== null ? A.fromPartial(t.config) : null, e.driveType = t.driveType ?? 0, e;
  }
};
function Q(t) {
  return t != null;
}
function $t() {
  return { x: 0, y: 0 };
}
const O = {
  encode(t, e = new w()) {
    return t.x !== 0 && e.uint32(9).double(t.x), t.y !== 0 && e.uint32(17).double(t.y), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = $t();
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
  fromJSON(t) {
    return {
      x: P(t.x) ? globalThis.Number(t.x) : 0,
      y: P(t.y) ? globalThis.Number(t.y) : 0
    };
  },
  toJSON(t) {
    const e = {};
    return t.x !== 0 && (e.x = t.x), t.y !== 0 && (e.y = t.y), e;
  },
  create(t) {
    return O.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = $t();
    return e.x = t.x ?? 0, e.y = t.y ?? 0, e;
  }
};
function Xt() {
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
    fl: null,
    fr: null,
    bl: null,
    br: null
  };
}
const _ = {
  encode(t, e = new w()) {
    return t.t !== 0 && e.uint32(9).double(t.t), t.x !== 0 && e.uint32(17).double(t.x), t.y !== 0 && e.uint32(25).double(t.y), t.heading !== 0 && e.uint32(33).double(t.heading), t.vx !== 0 && e.uint32(41).double(t.vx), t.vy !== 0 && e.uint32(49).double(t.vy), t.omega !== 0 && e.uint32(57).double(t.omega), t.ax !== 0 && e.uint32(65).double(t.ax), t.ay !== 0 && e.uint32(73).double(t.ay), t.alpha !== 0 && e.uint32(81).double(t.alpha), t.fl !== void 0 && t.fl !== null && O.encode(t.fl, e.uint32(90).fork()).join(), t.fr !== void 0 && t.fr !== null && O.encode(t.fr, e.uint32(98).fork()).join(), t.bl !== void 0 && t.bl !== null && O.encode(t.bl, e.uint32(106).fork()).join(), t.br !== void 0 && t.br !== null && O.encode(t.br, e.uint32(114).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Xt();
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
          i.fl = O.decode(r, r.uint32());
          continue;
        }
        case 12: {
          if (n !== 98)
            break;
          i.fr = O.decode(r, r.uint32());
          continue;
        }
        case 13: {
          if (n !== 106)
            break;
          i.bl = O.decode(r, r.uint32());
          continue;
        }
        case 14: {
          if (n !== 114)
            break;
          i.br = O.decode(r, r.uint32());
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
      t: P(t.t) ? globalThis.Number(t.t) : 0,
      x: P(t.x) ? globalThis.Number(t.x) : 0,
      y: P(t.y) ? globalThis.Number(t.y) : 0,
      heading: P(t.heading) ? globalThis.Number(t.heading) : 0,
      vx: P(t.vx) ? globalThis.Number(t.vx) : 0,
      vy: P(t.vy) ? globalThis.Number(t.vy) : 0,
      omega: P(t.omega) ? globalThis.Number(t.omega) : 0,
      ax: P(t.ax) ? globalThis.Number(t.ax) : 0,
      ay: P(t.ay) ? globalThis.Number(t.ay) : 0,
      alpha: P(t.alpha) ? globalThis.Number(t.alpha) : 0,
      fl: P(t.fl) ? O.fromJSON(t.fl) : null,
      fr: P(t.fr) ? O.fromJSON(t.fr) : null,
      bl: P(t.bl) ? O.fromJSON(t.bl) : null,
      br: P(t.br) ? O.fromJSON(t.br) : null
    };
  },
  toJSON(t) {
    const e = {};
    return t.t !== 0 && (e.t = t.t), t.x !== 0 && (e.x = t.x), t.y !== 0 && (e.y = t.y), t.heading !== 0 && (e.heading = t.heading), t.vx !== 0 && (e.vx = t.vx), t.vy !== 0 && (e.vy = t.vy), t.omega !== 0 && (e.omega = t.omega), t.ax !== 0 && (e.ax = t.ax), t.ay !== 0 && (e.ay = t.ay), t.alpha !== 0 && (e.alpha = t.alpha), t.fl !== void 0 && t.fl !== null && (e.fl = O.toJSON(t.fl)), t.fr !== void 0 && t.fr !== null && (e.fr = O.toJSON(t.fr)), t.bl !== void 0 && t.bl !== null && (e.bl = O.toJSON(t.bl)), t.br !== void 0 && t.br !== null && (e.br = O.toJSON(t.br)), e;
  },
  create(t) {
    return _.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Xt();
    return e.t = t.t ?? 0, e.x = t.x ?? 0, e.y = t.y ?? 0, e.heading = t.heading ?? 0, e.vx = t.vx ?? 0, e.vy = t.vy ?? 0, e.omega = t.omega ?? 0, e.ax = t.ax ?? 0, e.ay = t.ay ?? 0, e.alpha = t.alpha ?? 0, e.fl = t.fl !== void 0 && t.fl !== null ? O.fromPartial(t.fl) : null, e.fr = t.fr !== void 0 && t.fr !== null ? O.fromPartial(t.fr) : null, e.bl = t.bl !== void 0 && t.bl !== null ? O.fromPartial(t.bl) : null, e.br = t.br !== void 0 && t.br !== null ? O.fromPartial(t.br) : null, e;
  }
};
function P(t) {
  return t != null;
}
function qt() {
  return { samples: [] };
}
const K = {
  encode(t, e = new w()) {
    for (const r of t.samples)
      _.encode(r, e.uint32(10).fork()).join();
    return e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = qt();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.samples.push(_.decode(r, r.uint32()));
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
      samples: globalThis.Array.isArray(t?.samples) ? t.samples.map((e) => _.fromJSON(e)) : []
    };
  },
  toJSON(t) {
    const e = {};
    return t.samples?.length && (e.samples = t.samples.map((r) => _.toJSON(r))), e;
  },
  create(t) {
    return K.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = qt();
    return e.samples = t.samples?.map((r) => _.fromPartial(r)) || [], e;
  }
};
function Wt() {
  return { samples: [] };
}
const Y = {
  encode(t, e = new w()) {
    for (const r of t.samples)
      U.encode(r, e.uint32(10).fork()).join();
    return e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Wt();
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
  fromJSON(t) {
    return {
      samples: globalThis.Array.isArray(t?.samples) ? t.samples.map((e) => U.fromJSON(e)) : []
    };
  },
  toJSON(t) {
    const e = {};
    return t.samples?.length && (e.samples = t.samples.map((r) => U.toJSON(r))), e;
  },
  create(t) {
    return Y.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Wt();
    return e.samples = t.samples?.map((r) => U.fromPartial(r)) || [], e;
  }
};
function Kt() {
  return { trajectory: null, splits: [], waypoints: [], config: null };
}
const Z = {
  encode(t, e = new w()) {
    switch (t.trajectory?.$case) {
      case "swerve":
        K.encode(t.trajectory.value, e.uint32(10).fork()).join();
        break;
      case "differential":
        Y.encode(t.trajectory.value, e.uint32(18).fork()).join();
        break;
    }
    e.uint32(26).fork();
    for (const r of t.splits)
      e.uint64(r);
    e.join(), e.uint32(34).fork();
    for (const r of t.waypoints)
      e.double(r);
    return e.join(), t.config !== void 0 && t.config !== null && A.encode(t.config, e.uint32(42).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Kt();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.trajectory = { $case: "swerve", value: K.decode(r, r.uint32()) };
          continue;
        }
        case 2: {
          if (n !== 18)
            break;
          i.trajectory = { $case: "differential", value: Y.decode(r, r.uint32()) };
          continue;
        }
        case 3: {
          if (n === 24) {
            i.splits.push(Yt(r.uint64()));
            continue;
          }
          if (n === 26) {
            const s = r.uint32() + r.pos;
            for (; r.pos < s; )
              i.splits.push(Yt(r.uint64()));
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
        case 5: {
          if (n !== 42)
            break;
          i.config = A.decode(r, r.uint32());
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
      trajectory: ht(t.swerve) ? { $case: "swerve", value: K.fromJSON(t.swerve) } : ht(t.differential) ? { $case: "differential", value: Y.fromJSON(t.differential) } : null,
      splits: globalThis.Array.isArray(t?.splits) ? t.splits.map((e) => globalThis.Number(e)) : [],
      waypoints: globalThis.Array.isArray(t?.waypoints) ? t.waypoints.map((e) => globalThis.Number(e)) : [],
      config: ht(t.config) ? A.fromJSON(t.config) : null
    };
  },
  toJSON(t) {
    const e = {};
    return t.trajectory?.$case === "swerve" ? e.swerve = K.toJSON(t.trajectory.value) : t.trajectory?.$case === "differential" && (e.differential = Y.toJSON(t.trajectory.value)), t.splits?.length && (e.splits = t.splits.map((r) => Math.round(r))), t.waypoints?.length && (e.waypoints = t.waypoints), t.config !== void 0 && t.config !== null && (e.config = A.toJSON(t.config)), e;
  },
  create(t) {
    return Z.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Kt();
    switch (t.trajectory?.$case) {
      case "swerve": {
        t.trajectory?.value !== void 0 && t.trajectory?.value !== null && (e.trajectory = { $case: "swerve", value: K.fromPartial(t.trajectory.value) });
        break;
      }
      case "differential": {
        t.trajectory?.value !== void 0 && t.trajectory?.value !== null && (e.trajectory = {
          $case: "differential",
          value: Y.fromPartial(t.trajectory.value)
        });
        break;
      }
    }
    return e.splits = t.splits?.map((r) => r) || [], e.waypoints = t.waypoints?.map((r) => r) || [], e.config = t.config !== void 0 && t.config !== null ? A.fromPartial(t.config) : void 0, e;
  }
};
function Yt(t) {
  const e = globalThis.Number(t.toString());
  if (e > globalThis.Number.MAX_SAFE_INTEGER)
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  if (e < globalThis.Number.MIN_SAFE_INTEGER)
    throw new globalThis.Error("Value is smaller than Number.MIN_SAFE_INTEGER");
  return e;
}
function ht(t) {
  return t != null;
}
function Zt() {
  return { name: "", params: null, snapshot: null, trajectory: null };
}
const T = {
  encode(t, e = new w()) {
    return t.name !== "" && e.uint32(10).string(t.name), t.params !== void 0 && t.params !== null && I.encode(t.params, e.uint32(18).fork()).join(), t.snapshot !== void 0 && t.snapshot !== null && I.encode(t.snapshot, e.uint32(26).fork()).join(), t.trajectory !== void 0 && t.trajectory !== null && Z.encode(t.trajectory, e.uint32(34).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Zt();
    for (; r.pos < u; ) {
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
          i.params = I.decode(r, r.uint32());
          continue;
        }
        case 3: {
          if (n !== 26)
            break;
          i.snapshot = I.decode(r, r.uint32());
          continue;
        }
        case 4: {
          if (n !== 34)
            break;
          i.trajectory = Z.decode(r, r.uint32());
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
      name: j(t.name) ? globalThis.String(t.name) : "",
      params: j(t.params) ? I.fromJSON(t.params) : null,
      snapshot: j(t.snapshot) ? I.fromJSON(t.snapshot) : null,
      trajectory: j(t.trajectory) ? Z.fromJSON(t.trajectory) : null
    };
  },
  toJSON(t) {
    const e = {};
    return t.name !== "" && (e.name = t.name), t.params !== void 0 && t.params !== null && (e.params = I.toJSON(t.params)), t.snapshot !== void 0 && t.snapshot !== null && (e.snapshot = I.toJSON(t.snapshot)), t.trajectory !== void 0 && t.trajectory !== null && (e.trajectory = Z.toJSON(t.trajectory)), e;
  },
  create(t) {
    return T.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Zt();
    return e.name = t.name ?? "", e.params = t.params !== void 0 && t.params !== null ? I.fromPartial(t.params) : null, e.snapshot = t.snapshot !== void 0 && t.snapshot !== null ? I.fromPartial(t.snapshot) : void 0, e.trajectory = t.trajectory !== void 0 && t.trajectory !== null ? Z.fromPartial(t.trajectory) : void 0, e;
  }
};
function j(t) {
  return t != null;
}
const je = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DifferentialSample: U,
  DifferentialTrajectory: Y,
  DriveType: fe,
  Expr: y,
  ForceVector: O,
  GenerationOutput: Z,
  ProjectFile: pe,
  SwerveSample: _,
  SwerveTrajectory: K,
  TrajectoryFile: T,
  driveTypeFromJSON: pt,
  driveTypeToJSON: ce,
  parameters: Ue
}, Symbol.toStringTag, { value: "Module" }));
function Qt() {
  return { sample: null };
}
const it = {
  encode(t, e = new w()) {
    return t.sample !== void 0 && t.sample !== null && _.encode(t.sample, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = Qt();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.sample = _.decode(r, r.uint32());
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
    return { sample: ve(t.sample) ? _.fromJSON(t.sample) : null };
  },
  toJSON(t) {
    const e = {};
    return t.sample !== void 0 && t.sample !== null && (e.sample = _.toJSON(t.sample)), e;
  },
  create(t) {
    return it.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = Qt();
    return e.sample = t.sample !== void 0 && t.sample !== null ? _.fromPartial(t.sample) : null, e;
  }
};
function jt() {
  return { sample: null };
}
const yt = {
  encode(t, e = new w()) {
    return t.sample !== void 0 && t.sample !== null && _.encode(t.sample, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = jt();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.sample = _.decode(r, r.uint32());
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
    return { sample: ve(t.sample) ? _.fromJSON(t.sample) : null };
  },
  toJSON(t) {
    const e = {};
    return t.sample !== void 0 && t.sample !== null && (e.sample = _.toJSON(t.sample)), e;
  },
  create(t) {
    return yt.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = jt();
    return e.sample = t.sample !== void 0 && t.sample !== null ? _.fromPartial(t.sample) : null, e;
  }
};
function ve(t) {
  return t != null;
}
function te() {
  return { trajectory: null };
}
const ot = {
  encode(t, e = new w()) {
    return t.trajectory !== void 0 && t.trajectory !== null && T.encode(t.trajectory, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = te();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.trajectory = T.decode(r, r.uint32());
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
    return { trajectory: ye(t.trajectory) ? T.fromJSON(t.trajectory) : null };
  },
  toJSON(t) {
    const e = {};
    return t.trajectory !== void 0 && t.trajectory !== null && (e.trajectory = T.toJSON(t.trajectory)), e;
  },
  create(t) {
    return ot.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = te();
    return e.trajectory = t.trajectory !== void 0 && t.trajectory !== null ? T.fromPartial(t.trajectory) : null, e;
  }
};
function ee() {
  return { trajectory: null };
}
const bt = {
  encode(t, e = new w()) {
    return t.trajectory !== void 0 && t.trajectory !== null && T.encode(t.trajectory, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = ee();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.trajectory = T.decode(r, r.uint32());
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
    return { trajectory: ye(t.trajectory) ? T.fromJSON(t.trajectory) : null };
  },
  toJSON(t) {
    const e = {};
    return t.trajectory !== void 0 && t.trajectory !== null && (e.trajectory = T.toJSON(t.trajectory)), e;
  },
  create(t) {
    return bt.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = ee();
    return e.trajectory = t.trajectory !== void 0 && t.trajectory !== null ? T.fromPartial(t.trajectory) : null, e;
  }
};
function ye(t) {
  return t != null;
}
function re() {
  return { trajectory: null };
}
const mt = {
  encode(t, e = new w()) {
    return t.trajectory !== void 0 && t.trajectory !== null && T.encode(t.trajectory, e.uint32(10).fork()).join(), e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = re();
    for (; r.pos < u; ) {
      const n = r.uint32();
      switch (n >>> 3) {
        case 1: {
          if (n !== 10)
            break;
          i.trajectory = T.decode(r, r.uint32());
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
    return { trajectory: ze(t.trajectory) ? T.fromJSON(t.trajectory) : null };
  },
  toJSON(t) {
    const e = {};
    return t.trajectory !== void 0 && t.trajectory !== null && (e.trajectory = T.toJSON(t.trajectory)), e;
  },
  create(t) {
    return mt.fromPartial(t ?? {});
  },
  fromPartial(t) {
    const e = re();
    return e.trajectory = t.trajectory !== void 0 && t.trajectory !== null ? T.fromPartial(t.trajectory) : null, e;
  }
};
function ze(t) {
  return t != null;
}
const Ge = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  EchoSwerveSampleRequest: it,
  EchoSwerveSampleResponse: yt,
  GenerateRequest: ot,
  GenerateResponse: bt,
  GetDefaultTrajectoryResponse: mt
}, Symbol.toStringTag, { value: "Module" }));
var rt = { exports: {} }, $e = rt.exports, ne;
function Xe() {
  return ne || (ne = 1, (function(t, e) {
    (function(r, u) {
      t.exports = u();
    })($e, (function() {
      return r = { 418: function(i, n) {
        (function(s, a) {
          for (var b in a) s[b] = a[b];
        })(n, (function(s) {
          var a = {};
          function b(l) {
            if (a[l]) return a[l].exports;
            var c = a[l] = { i: l, l: !1, exports: {} };
            return s[l].call(c.exports, c, c.exports, b), c.l = !0, c.exports;
          }
          return b.m = s, b.c = a, b.i = function(l) {
            return l;
          }, b.d = function(l, c, m) {
            b.o(l, c) || Object.defineProperty(l, c, { configurable: !1, enumerable: !0, get: m });
          }, b.n = function(l) {
            var c = l && l.__esModule ? function() {
              return l.default;
            } : function() {
              return l;
            };
            return b.d(c, "a", c), c;
          }, b.o = function(l, c) {
            return Object.prototype.hasOwnProperty.call(l, c);
          }, b.p = "", b(b.s = 1);
        })([function(s, a, b) {
          Object.defineProperty(a, "__esModule", { value: !0 });
          var l = b(3), c = (function() {
            function m(f, h) {
              f === void 0 && (f = {}), h === void 0 && (h = { splitValues: !1 });
              var p, v = this;
              this.headersMap = {}, f && (typeof Headers < "u" && f instanceof Headers ? l.getHeaderKeys(f).forEach((function(o) {
                l.getHeaderValues(f, o).forEach((function(d) {
                  h.splitValues ? v.append(o, l.splitHeaderValue(d)) : v.append(o, d);
                }));
              })) : typeof (p = f) == "object" && typeof p.headersMap == "object" && typeof p.forEach == "function" ? f.forEach((function(o, d) {
                v.append(o, d);
              })) : typeof Map < "u" && f instanceof Map ? f.forEach((function(o, d) {
                v.append(d, o);
              })) : typeof f == "string" ? this.appendFromString(f) : typeof f == "object" && Object.getOwnPropertyNames(f).forEach((function(o) {
                var d = f[o];
                Array.isArray(d) ? d.forEach((function(x) {
                  v.append(o, x);
                })) : v.append(o, d);
              })));
            }
            return m.prototype.appendFromString = function(f) {
              for (var h = f.split(`\r
`), p = 0; p < h.length; p++) {
                var v = h[p], o = v.indexOf(":");
                if (o > 0) {
                  var d = v.substring(0, o).trim(), x = v.substring(o + 1).trim();
                  this.append(d, x);
                }
              }
            }, m.prototype.delete = function(f, h) {
              var p = l.normalizeName(f);
              if (h === void 0) delete this.headersMap[p];
              else {
                var v = this.headersMap[p];
                if (v) {
                  var o = v.indexOf(h);
                  o >= 0 && v.splice(o, 1), v.length === 0 && delete this.headersMap[p];
                }
              }
            }, m.prototype.append = function(f, h) {
              var p = this, v = l.normalizeName(f);
              Array.isArray(this.headersMap[v]) || (this.headersMap[v] = []), Array.isArray(h) ? h.forEach((function(o) {
                p.headersMap[v].push(l.normalizeValue(o));
              })) : this.headersMap[v].push(l.normalizeValue(h));
            }, m.prototype.set = function(f, h) {
              var p = l.normalizeName(f);
              if (Array.isArray(h)) {
                var v = [];
                h.forEach((function(o) {
                  v.push(l.normalizeValue(o));
                })), this.headersMap[p] = v;
              } else this.headersMap[p] = [l.normalizeValue(h)];
            }, m.prototype.has = function(f, h) {
              var p = this.headersMap[l.normalizeName(f)];
              if (!Array.isArray(p)) return !1;
              if (h !== void 0) {
                var v = l.normalizeValue(h);
                return p.indexOf(v) >= 0;
              }
              return !0;
            }, m.prototype.get = function(f) {
              var h = this.headersMap[l.normalizeName(f)];
              return h !== void 0 ? h.concat() : [];
            }, m.prototype.forEach = function(f) {
              var h = this;
              Object.getOwnPropertyNames(this.headersMap).forEach((function(p) {
                f(p, h.headersMap[p]);
              }), this);
            }, m.prototype.toHeaders = function() {
              if (typeof Headers < "u") {
                var f = new Headers();
                return this.forEach((function(h, p) {
                  p.forEach((function(v) {
                    f.append(h, v);
                  }));
                })), f;
              }
              throw new Error("Headers class is not defined");
            }, m;
          })();
          a.BrowserHeaders = c;
        }, function(s, a, b) {
          Object.defineProperty(a, "__esModule", { value: !0 });
          var l = b(0);
          a.BrowserHeaders = l.BrowserHeaders;
        }, function(s, a, b) {
          Object.defineProperty(a, "__esModule", { value: !0 }), a.iterateHeaders = function(l, c) {
            for (var m = l[Symbol.iterator](), f = m.next(); !f.done; ) c(f.value[0]), f = m.next();
          }, a.iterateHeadersKeys = function(l, c) {
            for (var m = l.keys(), f = m.next(); !f.done; ) c(f.value), f = m.next();
          };
        }, function(s, a, b) {
          Object.defineProperty(a, "__esModule", { value: !0 });
          var l = b(2);
          a.normalizeName = function(c) {
            if (typeof c != "string" && (c = String(c)), /[^a-z0-9\-#$%&'*+.\^_`|~]/i.test(c)) throw new TypeError("Invalid character in header field name");
            return c.toLowerCase();
          }, a.normalizeValue = function(c) {
            return typeof c != "string" && (c = String(c)), c;
          }, a.getHeaderValues = function(c, m) {
            var f = c;
            if (f instanceof Headers && f.getAll) return f.getAll(m);
            var h = f.get(m);
            return h && typeof h == "string" ? [h] : h;
          }, a.getHeaderKeys = function(c) {
            var m = c, f = {}, h = [];
            return m.keys ? l.iterateHeadersKeys(m, (function(p) {
              f[p] || (f[p] = !0, h.push(p));
            })) : m.forEach ? m.forEach((function(p, v) {
              f[v] || (f[v] = !0, h.push(v));
            })) : l.iterateHeaders(m, (function(p) {
              var v = p[0];
              f[v] || (f[v] = !0, h.push(v));
            })), h;
          }, a.splitHeaderValue = function(c) {
            var m = [];
            return c.split(", ").forEach((function(f) {
              f.split(",").forEach((function(h) {
                m.push(h);
              }));
            })), m;
          };
        }]));
      }, 617: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.ChunkParser = n.ChunkType = n.encodeASCII = n.decodeASCII = void 0;
        var a, b = s(65);
        function l(o) {
          return (d = o) === 9 || d === 10 || d === 13 || o >= 32 && o <= 126;
          var d;
        }
        function c(o) {
          for (var d = 0; d !== o.length; ++d) if (!l(o[d])) throw new Error("Metadata is not valid (printable) ASCII");
          return String.fromCharCode.apply(String, Array.prototype.slice.call(o));
        }
        function m(o) {
          return (128 & o.getUint8(0)) == 128;
        }
        function f(o) {
          return o.getUint32(1, !1);
        }
        function h(o, d, x) {
          return o.byteLength - d >= x;
        }
        function p(o, d, x) {
          if (o.slice) return o.slice(d, x);
          var k = o.length;
          x !== void 0 && (k = x);
          for (var g = new Uint8Array(k - d), M = 0, L = d; L < k; L++) g[M++] = o[L];
          return g;
        }
        n.decodeASCII = c, n.encodeASCII = function(o) {
          for (var d = new Uint8Array(o.length), x = 0; x !== o.length; ++x) {
            var k = o.charCodeAt(x);
            if (!l(k)) throw new Error("Metadata contains invalid ASCII");
            d[x] = k;
          }
          return d;
        }, (function(o) {
          o[o.MESSAGE = 1] = "MESSAGE", o[o.TRAILERS = 2] = "TRAILERS";
        })(a = n.ChunkType || (n.ChunkType = {}));
        var v = (function() {
          function o() {
            this.buffer = null, this.position = 0;
          }
          return o.prototype.parse = function(d, x) {
            if (d.length === 0 && x) return [];
            var k, g = [];
            if (this.buffer == null) this.buffer = d, this.position = 0;
            else if (this.position === this.buffer.byteLength) this.buffer = d, this.position = 0;
            else {
              var M = this.buffer.byteLength - this.position, L = new Uint8Array(M + d.byteLength), Se = p(this.buffer, this.position);
              L.set(Se, 0);
              var ge = new Uint8Array(d);
              L.set(ge, M), this.buffer = L, this.position = 0;
            }
            for (; ; ) {
              if (!h(this.buffer, this.position, 5)) return g;
              var ut = p(this.buffer, this.position, this.position + 5), xt = new DataView(ut.buffer, ut.byteOffset, ut.byteLength), lt = f(xt);
              if (!h(this.buffer, this.position, 5 + lt)) return g;
              var kt = p(this.buffer, this.position + 5, this.position + 5 + lt);
              if (this.position += 5 + lt, m(xt)) return g.push({ chunkType: a.TRAILERS, trailers: (k = kt, new b.Metadata(c(k))) }), g;
              g.push({ chunkType: a.MESSAGE, data: kt });
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
        var a = s(65), b = s(617), l = s(8), c = s(346), m = s(57), f = s(882);
        n.client = function(v, o) {
          return new h(v, o);
        };
        var h = (function() {
          function v(o, d) {
            this.started = !1, this.sentFirstMessage = !1, this.completed = !1, this.closed = !1, this.finishedSending = !1, this.onHeadersCallbacks = [], this.onMessageCallbacks = [], this.onEndCallbacks = [], this.parser = new b.ChunkParser(), this.methodDefinition = o, this.props = d, this.createTransport();
          }
          return v.prototype.createTransport = function() {
            var o = this.props.host + "/" + this.methodDefinition.service.serviceName + "/" + this.methodDefinition.methodName, d = { methodDefinition: this.methodDefinition, debug: this.props.debug || !1, url: o, onHeaders: this.onTransportHeaders.bind(this), onChunk: this.onTransportChunk.bind(this), onEnd: this.onTransportEnd.bind(this) };
            this.props.transport ? this.transport = this.props.transport(d) : this.transport = m.makeDefaultTransport(d);
          }, v.prototype.onTransportHeaders = function(o, d) {
            if (this.props.debug && c.debug("onHeaders", o, d), this.closed) this.props.debug && c.debug("grpc.onHeaders received after request was closed - ignoring");
            else if (d !== 0) {
              this.responseHeaders = o, this.props.debug && c.debug("onHeaders.responseHeaders", JSON.stringify(this.responseHeaders, null, 2));
              var x = p(o);
              this.props.debug && c.debug("onHeaders.gRPCStatus", x);
              var k = x && x >= 0 ? x : l.httpStatusToCode(d);
              this.props.debug && c.debug("onHeaders.code", k);
              var g = o.get("grpc-message") || [];
              if (this.props.debug && c.debug("onHeaders.gRPCMessage", g), this.rawOnHeaders(o), k !== l.Code.OK) {
                var M = this.decodeGRPCStatus(g[0]);
                this.rawOnError(k, M, o);
              }
            }
          }, v.prototype.onTransportChunk = function(o) {
            var d = this;
            if (this.closed) this.props.debug && c.debug("grpc.onChunk received after request was closed - ignoring");
            else {
              var x = [];
              try {
                x = this.parser.parse(o);
              } catch (k) {
                return this.props.debug && c.debug("onChunk.parsing error", k, k.message), void this.rawOnError(l.Code.Internal, "parsing error: " + k.message);
              }
              x.forEach((function(k) {
                if (k.chunkType === b.ChunkType.MESSAGE) {
                  var g = d.methodDefinition.responseType.deserializeBinary(k.data);
                  d.rawOnMessage(g);
                } else k.chunkType === b.ChunkType.TRAILERS && (d.responseHeaders ? (d.responseTrailers = new a.Metadata(k.trailers), d.props.debug && c.debug("onChunk.trailers", d.responseTrailers)) : (d.responseHeaders = new a.Metadata(k.trailers), d.rawOnHeaders(d.responseHeaders)));
              }));
            }
          }, v.prototype.onTransportEnd = function() {
            if (this.props.debug && c.debug("grpc.onEnd"), this.closed) this.props.debug && c.debug("grpc.onEnd received after request was closed - ignoring");
            else if (this.responseTrailers !== void 0) {
              var o = p(this.responseTrailers);
              if (o !== null) {
                var d = this.responseTrailers.get("grpc-message"), x = this.decodeGRPCStatus(d[0]);
                this.rawOnEnd(o, x, this.responseTrailers);
              } else this.rawOnError(l.Code.Internal, "Response closed without grpc-status (Trailers provided)");
            } else {
              if (this.responseHeaders === void 0) return void this.rawOnError(l.Code.Unknown, "Response closed without headers");
              var k = p(this.responseHeaders), g = this.responseHeaders.get("grpc-message");
              if (this.props.debug && c.debug("grpc.headers only response ", k, g), k === null) return void this.rawOnEnd(l.Code.Unknown, "Response closed without grpc-status (Headers only)", this.responseHeaders);
              var M = this.decodeGRPCStatus(g[0]);
              this.rawOnEnd(k, M, this.responseHeaders);
            }
          }, v.prototype.decodeGRPCStatus = function(o) {
            if (!o) return "";
            try {
              return decodeURIComponent(o);
            } catch {
              return o;
            }
          }, v.prototype.rawOnEnd = function(o, d, x) {
            var k = this;
            this.props.debug && c.debug("rawOnEnd", o, d, x), this.completed || (this.completed = !0, this.onEndCallbacks.forEach((function(g) {
              if (!k.closed) try {
                g(o, d, x);
              } catch (M) {
                setTimeout((function() {
                  throw M;
                }), 0);
              }
            })));
          }, v.prototype.rawOnHeaders = function(o) {
            this.props.debug && c.debug("rawOnHeaders", o), this.completed || this.onHeadersCallbacks.forEach((function(d) {
              try {
                d(o);
              } catch (x) {
                setTimeout((function() {
                  throw x;
                }), 0);
              }
            }));
          }, v.prototype.rawOnError = function(o, d, x) {
            var k = this;
            x === void 0 && (x = new a.Metadata()), this.props.debug && c.debug("rawOnError", o, d), this.completed || (this.completed = !0, this.onEndCallbacks.forEach((function(g) {
              if (!k.closed) try {
                g(o, d, x);
              } catch (M) {
                setTimeout((function() {
                  throw M;
                }), 0);
              }
            })));
          }, v.prototype.rawOnMessage = function(o) {
            var d = this;
            this.props.debug && c.debug("rawOnMessage", o.toObject()), this.completed || this.closed || this.onMessageCallbacks.forEach((function(x) {
              if (!d.closed) try {
                x(o);
              } catch (k) {
                setTimeout((function() {
                  throw k;
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
            var d = new a.Metadata(o || {});
            d.set("content-type", "application/grpc-web+proto"), d.set("x-grpc-web", "1"), this.transport.start(d);
          }, v.prototype.send = function(o) {
            if (!this.started) throw new Error("Client not started - .start() must be called before .send()");
            if (this.closed) throw new Error("Client already closed - cannot .send()");
            if (this.finishedSending) throw new Error("Client already finished sending - cannot .send()");
            if (!this.methodDefinition.requestStream && this.sentFirstMessage) throw new Error("Message already sent for non-client-streaming method - cannot .send()");
            this.sentFirstMessage = !0;
            var d = f.frameRequest(o);
            this.transport.sendMessage(d);
          }, v.prototype.finishSend = function() {
            if (!this.started) throw new Error("Client not started - .finishSend() must be called before .close()");
            if (this.closed) throw new Error("Client already closed - cannot .send()");
            if (this.finishedSending) throw new Error("Client already finished sending - cannot .finishSend()");
            this.finishedSending = !0, this.transport.finishSend();
          }, v.prototype.close = function() {
            if (!this.started) throw new Error("Client not started - .start() must be called before .close()");
            if (this.closed) throw new Error("Client already closed - cannot .close()");
            this.closed = !0, this.props.debug && c.debug("request.abort aborting request"), this.transport.cancel();
          }, v;
        })();
        function p(v) {
          var o = v.get("grpc-status") || [];
          if (o.length > 0) try {
            var d = o[0];
            return parseInt(d, 10);
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
        var a, b = s(418), l = s(57), c = s(229), m = s(540), f = s(210), h = s(859), p = s(8), v = s(938), o = s(35), d = s(934);
        (a = n.grpc || (n.grpc = {})).setDefaultTransport = l.setDefaultTransportFactory, a.CrossBrowserHttpTransport = h.CrossBrowserHttpTransport, a.FetchReadableStreamTransport = c.FetchReadableStreamTransport, a.XhrTransport = f.XhrTransport, a.WebsocketTransport = m.WebsocketTransport, a.Code = p.Code, a.Metadata = b.BrowserHeaders, a.client = function(x, k) {
          return d.client(x, k);
        }, a.invoke = v.invoke, a.unary = o.unary;
      }, 938: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.invoke = void 0;
        var a = s(934);
        n.invoke = function(b, l) {
          if (b.requestStream) throw new Error(".invoke cannot be used with client-streaming methods. Use .client instead.");
          var c = a.client(b, { host: l.host, transport: l.transport, debug: l.debug });
          return l.onHeaders && c.onHeaders(l.onHeaders), l.onMessage && c.onMessage(l.onMessage), l.onEnd && c.onEnd(l.onEnd), c.start(l.metadata), c.send(l.request), c.finishSend(), { close: function() {
            c.close();
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
        var a = s(859), b = function(l) {
          return a.CrossBrowserHttpTransport({ withCredentials: !1 })(l);
        };
        n.setDefaultTransportFactory = function(l) {
          b = l;
        }, n.makeDefaultTransport = function(l) {
          return b(l);
        };
      }, 229: function(i, n, s) {
        var a = this && this.__assign || function() {
          return (a = Object.assign || function(m) {
            for (var f, h = 1, p = arguments.length; h < p; h++) for (var v in f = arguments[h]) Object.prototype.hasOwnProperty.call(f, v) && (m[v] = f[v]);
            return m;
          }).apply(this, arguments);
        };
        Object.defineProperty(n, "__esModule", { value: !0 }), n.detectFetchSupport = n.FetchReadableStreamTransport = void 0;
        var b = s(65), l = s(346);
        n.FetchReadableStreamTransport = function(m) {
          return function(f) {
            return (function(h, p) {
              return h.debug && l.debug("fetchRequest", h), new c(h, p);
            })(f, m);
          };
        };
        var c = (function() {
          function m(f, h) {
            this.cancelled = !1, this.controller = self.AbortController && new AbortController(), this.options = f, this.init = h;
          }
          return m.prototype.pump = function(f, h) {
            var p = this;
            if (this.reader = f, this.cancelled) return this.options.debug && l.debug("Fetch.pump.cancel at first pump"), void this.reader.cancel().catch((function(v) {
              p.options.debug && l.debug("Fetch.pump.reader.cancel exception", v);
            }));
            this.reader.read().then((function(v) {
              if (v.done) return p.options.onEnd(), h;
              p.options.onChunk(v.value), p.pump(p.reader, h);
            })).catch((function(v) {
              p.cancelled ? p.options.debug && l.debug("Fetch.catch - request cancelled") : (p.cancelled = !0, p.options.debug && l.debug("Fetch.catch", v.message), p.options.onEnd(v));
            }));
          }, m.prototype.send = function(f) {
            var h = this;
            fetch(this.options.url, a(a({}, this.init), { headers: this.metadata.toHeaders(), method: "POST", body: f, signal: this.controller && this.controller.signal })).then((function(p) {
              if (h.options.debug && l.debug("Fetch.response", p), h.options.onHeaders(new b.Metadata(p.headers), p.status), !p.body) return p;
              h.pump(p.body.getReader(), p);
            })).catch((function(p) {
              h.cancelled ? h.options.debug && l.debug("Fetch.catch - request cancelled") : (h.cancelled = !0, h.options.debug && l.debug("Fetch.catch", p.message), h.options.onEnd(p));
            }));
          }, m.prototype.sendMessage = function(f) {
            this.send(f);
          }, m.prototype.finishSend = function() {
          }, m.prototype.start = function(f) {
            this.metadata = f;
          }, m.prototype.cancel = function() {
            var f = this;
            this.cancelled ? this.options.debug && l.debug("Fetch.cancel already cancelled") : (this.cancelled = !0, this.controller ? (this.options.debug && l.debug("Fetch.cancel.controller.abort"), this.controller.abort()) : this.options.debug && l.debug("Fetch.cancel.missing abort controller"), this.reader ? (this.options.debug && l.debug("Fetch.cancel.reader.cancel"), this.reader.cancel().catch((function(h) {
              f.options.debug && l.debug("Fetch.cancel.reader.cancel exception", h);
            }))) : this.options.debug && l.debug("Fetch.cancel before reader"));
          }, m;
        })();
        n.detectFetchSupport = function() {
          return typeof Response < "u" && Response.prototype.hasOwnProperty("body") && typeof Headers == "function";
        };
      }, 859: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.CrossBrowserHttpTransport = void 0;
        var a = s(229), b = s(210);
        n.CrossBrowserHttpTransport = function(l) {
          if (a.detectFetchSupport()) {
            var c = { credentials: l.withCredentials ? "include" : "same-origin" };
            return a.FetchReadableStreamTransport(c);
          }
          return b.XhrTransport({ withCredentials: l.withCredentials });
        };
      }, 210: function(i, n, s) {
        var a, b = this && this.__extends || (a = function(o, d) {
          return (a = Object.setPrototypeOf || { __proto__: [] } instanceof Array && function(x, k) {
            x.__proto__ = k;
          } || function(x, k) {
            for (var g in k) Object.prototype.hasOwnProperty.call(k, g) && (x[g] = k[g]);
          })(o, d);
        }, function(o, d) {
          function x() {
            this.constructor = o;
          }
          a(o, d), o.prototype = d === null ? Object.create(d) : (x.prototype = d.prototype, new x());
        });
        Object.defineProperty(n, "__esModule", { value: !0 }), n.stringToArrayBuffer = n.MozChunkedArrayBufferXHR = n.XHR = n.XhrTransport = void 0;
        var l = s(65), c = s(346), m = s(849);
        n.XhrTransport = function(o) {
          return function(d) {
            if (m.detectMozXHRSupport()) return new h(d, o);
            if (m.detectXHROverrideMimeTypeSupport()) return new f(d, o);
            throw new Error("This environment's XHR implementation cannot support binary transfer.");
          };
        };
        var f = (function() {
          function o(d, x) {
            this.options = d, this.init = x;
          }
          return o.prototype.onProgressEvent = function() {
            this.options.debug && c.debug("XHR.onProgressEvent.length: ", this.xhr.response.length);
            var d = this.xhr.response.substr(this.index);
            this.index = this.xhr.response.length;
            var x = v(d);
            this.options.onChunk(x);
          }, o.prototype.onLoadEvent = function() {
            this.options.debug && c.debug("XHR.onLoadEvent"), this.options.onEnd();
          }, o.prototype.onStateChange = function() {
            this.options.debug && c.debug("XHR.onStateChange", this.xhr.readyState), this.xhr.readyState === XMLHttpRequest.HEADERS_RECEIVED && this.options.onHeaders(new l.Metadata(this.xhr.getAllResponseHeaders()), this.xhr.status);
          }, o.prototype.sendMessage = function(d) {
            this.xhr.send(d);
          }, o.prototype.finishSend = function() {
          }, o.prototype.start = function(d) {
            var x = this;
            this.metadata = d;
            var k = new XMLHttpRequest();
            this.xhr = k, k.open("POST", this.options.url), this.configureXhr(), this.metadata.forEach((function(g, M) {
              k.setRequestHeader(g, M.join(", "));
            })), k.withCredentials = !!this.init.withCredentials, k.addEventListener("readystatechange", this.onStateChange.bind(this)), k.addEventListener("progress", this.onProgressEvent.bind(this)), k.addEventListener("loadend", this.onLoadEvent.bind(this)), k.addEventListener("error", (function(g) {
              x.options.debug && c.debug("XHR.error", g), x.options.onEnd(g.error);
            }));
          }, o.prototype.configureXhr = function() {
            this.xhr.responseType = "text", this.xhr.overrideMimeType("text/plain; charset=x-user-defined");
          }, o.prototype.cancel = function() {
            this.options.debug && c.debug("XHR.abort"), this.xhr.abort();
          }, o;
        })();
        n.XHR = f;
        var h = (function(o) {
          function d() {
            return o !== null && o.apply(this, arguments) || this;
          }
          return b(d, o), d.prototype.configureXhr = function() {
            this.options.debug && c.debug("MozXHR.configureXhr: setting responseType to 'moz-chunked-arraybuffer'"), this.xhr.responseType = "moz-chunked-arraybuffer";
          }, d.prototype.onProgressEvent = function() {
            var x = this.xhr.response;
            this.options.debug && c.debug("MozXHR.onProgressEvent: ", new Uint8Array(x)), this.options.onChunk(new Uint8Array(x));
          }, d;
        })(f);
        function p(o, d) {
          var x = o.charCodeAt(d);
          if (x >= 55296 && x <= 56319) {
            var k = o.charCodeAt(d + 1);
            k >= 56320 && k <= 57343 && (x = 65536 + (x - 55296 << 10) + (k - 56320));
          }
          return x;
        }
        function v(o) {
          for (var d = new Uint8Array(o.length), x = 0, k = 0; k < o.length; k++) {
            var g = String.prototype.codePointAt ? o.codePointAt(k) : p(o, k);
            d[x++] = 255 & g;
          }
          return d;
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
        function b(l) {
          var c = a();
          if (!c) return !1;
          try {
            return c.responseType = l, c.responseType === l;
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
        var a, b = s(346), l = s(617);
        (function(m) {
          m[m.FINISH_SEND = 1] = "FINISH_SEND";
        })(a || (a = {}));
        var c = new Uint8Array([1]);
        n.WebsocketTransport = function() {
          return function(m) {
            return (function(f) {
              f.debug && b.debug("websocketRequest", f);
              var h, p = (function(d) {
                if (d.substr(0, 8) === "https://") return "wss://" + d.substr(8);
                if (d.substr(0, 7) === "http://") return "ws://" + d.substr(7);
                throw new Error("Websocket transport constructed with non-https:// or http:// host.");
              })(f.url), v = [];
              function o(d) {
                if (d === a.FINISH_SEND) h.send(c);
                else {
                  var x = d, k = new Int8Array(x.byteLength + 1);
                  k.set(new Uint8Array([0])), k.set(x, 1), h.send(k);
                }
              }
              return { sendMessage: function(d) {
                h && h.readyState !== h.CONNECTING ? o(d) : v.push(d);
              }, finishSend: function() {
                h && h.readyState !== h.CONNECTING ? o(a.FINISH_SEND) : v.push(a.FINISH_SEND);
              }, start: function(d) {
                (h = new WebSocket(p, ["grpc-websockets"])).binaryType = "arraybuffer", h.onopen = function() {
                  var x;
                  f.debug && b.debug("websocketRequest.onopen"), h.send((x = "", d.forEach((function(k, g) {
                    x += k + ": " + g.join(", ") + `\r
`;
                  })), l.encodeASCII(x))), v.forEach((function(k) {
                    o(k);
                  }));
                }, h.onclose = function(x) {
                  f.debug && b.debug("websocketRequest.onclose", x), f.onEnd();
                }, h.onerror = function(x) {
                  f.debug && b.debug("websocketRequest.onerror", x);
                }, h.onmessage = function(x) {
                  f.onChunk(new Uint8Array(x.data));
                };
              }, cancel: function() {
                f.debug && b.debug("websocket.abort"), h.close();
              } };
            })(m);
          };
        };
      }, 35: function(i, n, s) {
        Object.defineProperty(n, "__esModule", { value: !0 }), n.unary = void 0;
        var a = s(65), b = s(934);
        n.unary = function(l, c) {
          if (l.responseStream) throw new Error(".unary cannot be used with server-streaming methods. Use .invoke or .client instead.");
          if (l.requestStream) throw new Error(".unary cannot be used with client-streaming methods. Use .client instead.");
          var m = null, f = null, h = b.client(l, { host: c.host, transport: c.transport, debug: c.debug });
          return h.onHeaders((function(p) {
            m = p;
          })), h.onMessage((function(p) {
            f = p;
          })), h.onEnd((function(p, v, o) {
            c.onEnd({ status: p, statusMessage: v, headers: m || new a.Metadata(), message: f, trailers: o });
          })), h.start(c.metadata), h.send(c.request), h.finishSend(), { close: function() {
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
  })(rt)), rt.exports;
}
var ie = Xe(), nt = { exports: {} }, qe = nt.exports, oe;
function We() {
  return oe || (oe = 1, (function(t, e) {
    (function(u, i) {
      t.exports = i();
    })(qe, function() {
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
              function b(l, c) {
                l === void 0 && (l = {}), c === void 0 && (c = { splitValues: !1 });
                var m = this;
                if (this.headersMap = {}, l)
                  if (typeof Headers < "u" && l instanceof Headers) {
                    var f = n.getHeaderKeys(l);
                    f.forEach(function(p) {
                      var v = n.getHeaderValues(l, p);
                      v.forEach(function(o) {
                        c.splitValues ? m.append(p, n.splitHeaderValue(o)) : m.append(p, o);
                      });
                    });
                  } else if (s(l))
                    l.forEach(function(p, v) {
                      m.append(p, v);
                    });
                  else if (typeof Map < "u" && l instanceof Map) {
                    var h = l;
                    h.forEach(function(p, v) {
                      m.append(v, p);
                    });
                  } else typeof l == "string" ? this.appendFromString(l) : typeof l == "object" && Object.getOwnPropertyNames(l).forEach(function(p) {
                    var v = l, o = v[p];
                    Array.isArray(o) ? o.forEach(function(d) {
                      m.append(p, d);
                    }) : m.append(p, o);
                  });
              }
              return b.prototype.appendFromString = function(l) {
                for (var c = l.split(`\r
`), m = 0; m < c.length; m++) {
                  var f = c[m], h = f.indexOf(":");
                  if (h > 0) {
                    var p = f.substring(0, h).trim(), v = f.substring(h + 1).trim();
                    this.append(p, v);
                  }
                }
              }, b.prototype.delete = function(l, c) {
                var m = n.normalizeName(l);
                if (c === void 0)
                  delete this.headersMap[m];
                else {
                  var f = this.headersMap[m];
                  if (f) {
                    var h = f.indexOf(c);
                    h >= 0 && f.splice(h, 1), f.length === 0 && delete this.headersMap[m];
                  }
                }
              }, b.prototype.append = function(l, c) {
                var m = this, f = n.normalizeName(l);
                Array.isArray(this.headersMap[f]) || (this.headersMap[f] = []), Array.isArray(c) ? c.forEach(function(h) {
                  m.headersMap[f].push(n.normalizeValue(h));
                }) : this.headersMap[f].push(n.normalizeValue(c));
              }, b.prototype.set = function(l, c) {
                var m = n.normalizeName(l);
                if (Array.isArray(c)) {
                  var f = [];
                  c.forEach(function(h) {
                    f.push(n.normalizeValue(h));
                  }), this.headersMap[m] = f;
                } else
                  this.headersMap[m] = [n.normalizeValue(c)];
              }, b.prototype.has = function(l, c) {
                var m = this.headersMap[n.normalizeName(l)], f = Array.isArray(m);
                if (!f)
                  return !1;
                if (c !== void 0) {
                  var h = n.normalizeValue(c);
                  return m.indexOf(h) >= 0;
                } else
                  return !0;
              }, b.prototype.get = function(l) {
                var c = this.headersMap[n.normalizeName(l)];
                return c !== void 0 ? c.concat() : [];
              }, b.prototype.forEach = function(l) {
                var c = this;
                Object.getOwnPropertyNames(this.headersMap).forEach(function(m) {
                  l(m, c.headersMap[m]);
                }, this);
              }, b.prototype.toHeaders = function() {
                if (typeof Headers < "u") {
                  var l = new Headers();
                  return this.forEach(function(c, m) {
                    m.forEach(function(f) {
                      l.append(c, f);
                    });
                  }), l;
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
              for (var l = a[Symbol.iterator](), c = l.next(); !c.done; )
                b(c.value[0]), c = l.next();
            }
            u.iterateHeaders = n;
            function s(a, b) {
              for (var l = a.keys(), c = l.next(); !c.done; )
                b(c.value), c = l.next();
            }
            u.iterateHeadersKeys = s;
          }),
          /* 3 */
          /***/
          (function(r, u, i) {
            Object.defineProperty(u, "__esModule", { value: !0 });
            var n = i(2);
            function s(f) {
              if (typeof f != "string" && (f = String(f)), /[^a-z0-9\-#$%&'*+.\^_`|~]/i.test(f))
                throw new TypeError("Invalid character in header field name");
              return f.toLowerCase();
            }
            u.normalizeName = s;
            function a(f) {
              return typeof f != "string" && (f = String(f)), f;
            }
            u.normalizeValue = a;
            function b(f, h) {
              var p = f;
              if (p instanceof Headers && p.getAll)
                return p.getAll(h);
              var v = p.get(h);
              return v && typeof v == "string" ? [v] : v;
            }
            u.getHeaderValues = b;
            function l(f) {
              return f;
            }
            function c(f) {
              var h = f, p = {}, v = [];
              return h.keys ? n.iterateHeadersKeys(h, function(o) {
                p[o] || (p[o] = !0, v.push(o));
              }) : h.forEach ? h.forEach(function(o, d) {
                p[d] || (p[d] = !0, v.push(d));
              }) : n.iterateHeaders(h, function(o) {
                var d = o[0];
                p[d] || (p[d] = !0, v.push(d));
              }), v;
            }
            u.getHeaderKeys = c;
            function m(f) {
              var h = [], p = f.split(", ");
              return p.forEach(function(v) {
                v.split(",").forEach(function(o) {
                  h.push(o);
                });
              }), h;
            }
            u.splitHeaderValue = m;
          })
          /******/
        ])
      );
    });
  })(nt)), nt.exports;
}
var Ke = We();
function ae() {
  return {};
}
const at = {
  encode(t, e = new w()) {
    return e;
  },
  decode(t, e) {
    const r = t instanceof S ? t : new S(t), u = e === void 0 ? r.len : r.pos + e, i = ae();
    for (; r.pos < u; ) {
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
    return at.fromPartial(t ?? {});
  },
  fromPartial(t) {
    return ae();
  }
};
class Ye {
  rpc;
  constructor(e) {
    this.rpc = e, this.EchoSwerveSample = this.EchoSwerveSample.bind(this), this.Generate = this.Generate.bind(this), this.GetDefaultTrajectory = this.GetDefaultTrajectory.bind(this);
  }
  EchoSwerveSample(e, r) {
    return this.rpc.unary(be, it.fromPartial(e), r);
  }
  Generate(e, r) {
    return this.rpc.unary(me, ot.fromPartial(e), r);
  }
  GetDefaultTrajectory(e, r) {
    return this.rpc.unary(xe, at.fromPartial(e), r);
  }
}
const st = { serviceName: "service.ChoreoService" }, be = {
  methodName: "EchoSwerveSample",
  service: st,
  requestStream: !1,
  responseStream: !1,
  requestType: {
    serializeBinary() {
      return it.encode(this).finish();
    }
  },
  responseType: {
    deserializeBinary(t) {
      const e = yt.decode(t);
      return {
        ...e,
        toObject() {
          return e;
        }
      };
    }
  }
}, me = {
  methodName: "Generate",
  service: st,
  requestStream: !1,
  responseStream: !1,
  requestType: {
    serializeBinary() {
      return ot.encode(this).finish();
    }
  },
  responseType: {
    deserializeBinary(t) {
      const e = bt.decode(t);
      return {
        ...e,
        toObject() {
          return e;
        }
      };
    }
  }
}, xe = {
  methodName: "GetDefaultTrajectory",
  service: st,
  requestStream: !1,
  responseStream: !1,
  requestType: {
    serializeBinary() {
      return at.encode(this).finish();
    }
  },
  responseType: {
    deserializeBinary(t) {
      const e = mt.decode(t);
      return {
        ...e,
        toObject() {
          return e;
        }
      };
    }
  }
};
class Ze {
  host;
  options;
  constructor(e, r) {
    this.host = e, this.options = r;
  }
  unary(e, r, u) {
    const i = { ...r, ...e.requestType }, n = u && this.options.metadata ? new Ke.BrowserHeaders({ ...this.options?.metadata.headersMap, ...u?.headersMap }) : u ?? this.options.metadata;
    return new Promise((s, a) => {
      ie.grpc.unary(e, {
        request: i,
        host: this.host,
        metadata: n ?? {},
        ...this.options.transport !== void 0 ? { transport: this.options.transport } : {},
        debug: this.options.debug ?? !1,
        onEnd: function(b) {
          if (b.status === ie.grpc.Code.OK)
            s(b.message.toObject());
          else {
            const l = new ke(b.statusMessage, b.status, b.trailers);
            a(l);
          }
        }
      });
    });
  }
}
class ke extends globalThis.Error {
  constructor(e, r, u) {
    super(e), this.code = r, this.metadata = u;
  }
}
const tr = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  ChoreoServiceClientImpl: Ye,
  ChoreoServiceDesc: st,
  ChoreoServiceEchoSwerveSampleDesc: be,
  ChoreoServiceGenerateDesc: me,
  ChoreoServiceGetDefaultTrajectoryDesc: xe,
  GrpcWebError: ke,
  GrpcWebImpl: Ze,
  commands: Ge
}, Symbol.toStringTag, { value: "Module" })), Qe = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  Empty: at
}, Symbol.toStringTag, { value: "Module" })), er = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  protobuf: Qe
}, Symbol.toStringTag, { value: "Module" }));
export {
  je as entity,
  er as google,
  tr as service
};
//# sourceMappingURL=index.mjs.map
