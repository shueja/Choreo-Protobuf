function pe() {
  let e = 0, t = 0;
  for (let b = 0; b < 28; b += 7) {
    let c = this.buf[this.pos++];
    if (e |= (c & 127) << b, (c & 128) == 0)
      return this.assertBounds(), [e, t];
  }
  let n = this.buf[this.pos++];
  if (e |= (n & 15) << 28, t = (n & 112) >> 4, (n & 128) == 0)
    return this.assertBounds(), [e, t];
  for (let b = 3; b <= 31; b += 7) {
    let c = this.buf[this.pos++];
    if (t |= (c & 127) << b, (c & 128) == 0)
      return this.assertBounds(), [e, t];
  }
  throw new Error("invalid varint");
}
function U(e, t, n) {
  for (let r = 0; r < 28; r = r + 7) {
    const s = e >>> r, o = !(!(s >>> 7) && t == 0), y = (o ? s | 128 : s) & 255;
    if (n.push(y), !o)
      return;
  }
  const b = e >>> 28 & 15 | (t & 7) << 4, c = t >> 3 != 0;
  if (n.push((c ? b | 128 : b) & 255), !!c) {
    for (let r = 3; r < 31; r = r + 7) {
      const s = t >>> r, o = !!(s >>> 7), y = (o ? s | 128 : s) & 255;
      if (n.push(y), !o)
        return;
    }
    n.push(t >>> 31 & 1);
  }
}
const N = 4294967296;
function X(e) {
  const t = e[0] === "-";
  t && (e = e.slice(1));
  const n = 1e6;
  let b = 0, c = 0;
  function r(s, o) {
    const y = Number(e.slice(s, o));
    c *= n, b = b * n + y, b >= N && (c = c + (b / N | 0), b = b % N);
  }
  return r(-24, -18), r(-18, -12), r(-12, -6), r(-6), t ? oe(b, c) : z(b, c);
}
function be(e, t) {
  let n = z(e, t);
  const b = n.hi & 2147483648;
  b && (n = oe(n.lo, n.hi));
  const c = ie(n.lo, n.hi);
  return b ? "-" + c : c;
}
function ie(e, t) {
  if ({ lo: e, hi: t } = ye(e, t), t <= 2097151)
    return String(N * t + e);
  const n = e & 16777215, b = (e >>> 24 | t << 8) & 16777215, c = t >> 16 & 65535;
  let r = n + b * 6777216 + c * 6710656, s = b + c * 8147497, o = c * 2;
  const y = 1e7;
  return r >= y && (s += Math.floor(r / y), r %= y), s >= y && (o += Math.floor(s / y), s %= y), o.toString() + J(s) + J(r);
}
function ye(e, t) {
  return { lo: e >>> 0, hi: t >>> 0 };
}
function z(e, t) {
  return { lo: e | 0, hi: t | 0 };
}
function oe(e, t) {
  return t = ~t, e ? e = ~e + 1 : t += 1, z(e, t);
}
const J = (e) => {
  const t = String(e);
  return "0000000".slice(t.length) + t;
};
function G(e, t) {
  if (e >= 0) {
    for (; e > 127; )
      t.push(e & 127 | 128), e = e >>> 7;
    t.push(e);
  } else {
    for (let n = 0; n < 9; n++)
      t.push(e & 127 | 128), e = e >> 7;
    t.push(1);
  }
}
function ve() {
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
  for (let n = 5; (e & 128) !== 0 && n < 10; n++)
    e = this.buf[this.pos++];
  if ((e & 128) != 0)
    throw new Error("invalid varint");
  return this.assertBounds(), t >>> 0;
}
const H = /* @__PURE__ */ ge();
function ge() {
  const e = new DataView(new ArrayBuffer(8));
  if (typeof BigInt == "function" && typeof e.getBigInt64 == "function" && typeof e.getBigUint64 == "function" && typeof e.setBigInt64 == "function" && typeof e.setBigUint64 == "function" && (!!globalThis.Deno || typeof process != "object" || typeof process.env != "object" || process.env.BUF_BIGINT_DISABLE !== "1")) {
    const n = BigInt("-9223372036854775808"), b = BigInt("9223372036854775807"), c = BigInt("0"), r = BigInt("18446744073709551615");
    return {
      zero: BigInt(0),
      supported: !0,
      parse(s) {
        const o = typeof s == "bigint" ? s : BigInt(s);
        if (o > b || o < n)
          throw new Error(`invalid int64: ${s}`);
        return o;
      },
      uParse(s) {
        const o = typeof s == "bigint" ? s : BigInt(s);
        if (o > r || o < c)
          throw new Error(`invalid uint64: ${s}`);
        return o;
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
      dec(s, o) {
        return e.setInt32(0, s, !0), e.setInt32(4, o, !0), e.getBigInt64(0, !0);
      },
      uDec(s, o) {
        return e.setInt32(0, s, !0), e.setInt32(4, o, !0), e.getBigUint64(0, !0);
      }
    };
  }
  return {
    zero: "0",
    supported: !1,
    parse(n) {
      return typeof n != "string" && (n = n.toString()), K(n), n;
    },
    uParse(n) {
      return typeof n != "string" && (n = n.toString()), W(n), n;
    },
    enc(n) {
      return typeof n != "string" && (n = n.toString()), K(n), X(n);
    },
    uEnc(n) {
      return typeof n != "string" && (n = n.toString()), W(n), X(n);
    },
    dec(n, b) {
      return be(n, b);
    },
    uDec(n, b) {
      return ie(n, b);
    }
  };
}
function K(e) {
  if (!/^-?[0-9]+$/.test(e))
    throw new Error("invalid int64: " + e);
}
function W(e) {
  if (!/^[0-9]+$/.test(e))
    throw new Error("invalid uint64: " + e);
}
const F = /* @__PURE__ */ Symbol.for("@bufbuild/protobuf/text-encoding");
function se() {
  if (globalThis[F] == null) {
    const e = new globalThis.TextEncoder(), t = new globalThis.TextDecoder();
    globalThis[F] = {
      encodeUtf8(n) {
        return e.encode(n);
      },
      decodeUtf8(n) {
        return t.decode(n);
      },
      checkUtf8(n) {
        try {
          return encodeURIComponent(n), !0;
        } catch {
          return !1;
        }
      }
    };
  }
  return globalThis[F];
}
var T;
(function(e) {
  e[e.Varint = 0] = "Varint", e[e.Bit64 = 1] = "Bit64", e[e.LengthDelimited = 2] = "LengthDelimited", e[e.StartGroup = 3] = "StartGroup", e[e.EndGroup = 4] = "EndGroup", e[e.Bit32 = 5] = "Bit32";
})(T || (T = {}));
const we = 34028234663852886e22, me = -34028234663852886e22, xe = 4294967295, Ee = 2147483647, Se = -2147483648;
class C {
  constructor(t = se().encodeUtf8) {
    this.encodeUtf8 = t, this.stack = [], this.chunks = [], this.buf = [];
  }
  /**
   * Return all bytes written and reset this writer.
   */
  finish() {
    this.buf.length && (this.chunks.push(new Uint8Array(this.buf)), this.buf = []);
    let t = 0;
    for (let c = 0; c < this.chunks.length; c++)
      t += this.chunks[c].length;
    let n = new Uint8Array(t), b = 0;
    for (let c = 0; c < this.chunks.length; c++)
      n.set(this.chunks[c], b), b += this.chunks[c].length;
    return this.chunks = [], n;
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
    let t = this.finish(), n = this.stack.pop();
    if (!n)
      throw new Error("invalid state, fork stack empty");
    return this.chunks = n.chunks, this.buf = n.buf, this.uint32(t.byteLength), this.raw(t);
  }
  /**
   * Writes a tag (field number and wire type).
   *
   * Equivalent to `uint32( (fieldNo << 3 | type) >>> 0 )`.
   *
   * Generated code should compute the tag ahead of time and call `uint32()`.
   */
  tag(t, n) {
    return this.uint32((t << 3 | n) >>> 0);
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
    for ($(t); t > 127; )
      this.buf.push(t & 127 | 128), t = t >>> 7;
    return this.buf.push(t), this;
  }
  /**
   * Write a `int32` value, a signed 32 bit varint.
   */
  int32(t) {
    return D(t), G(t, this.buf), this;
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
    let n = this.encodeUtf8(t);
    return this.uint32(n.byteLength), this.raw(n);
  }
  /**
   * Write a `float` value, 32-bit floating point number.
   */
  float(t) {
    ke(t);
    let n = new Uint8Array(4);
    return new DataView(n.buffer).setFloat32(0, t, !0), this.raw(n);
  }
  /**
   * Write a `double` value, a 64-bit floating point number.
   */
  double(t) {
    let n = new Uint8Array(8);
    return new DataView(n.buffer).setFloat64(0, t, !0), this.raw(n);
  }
  /**
   * Write a `fixed32` value, an unsigned, fixed-length 32-bit integer.
   */
  fixed32(t) {
    $(t);
    let n = new Uint8Array(4);
    return new DataView(n.buffer).setUint32(0, t, !0), this.raw(n);
  }
  /**
   * Write a `sfixed32` value, a signed, fixed-length 32-bit integer.
   */
  sfixed32(t) {
    D(t);
    let n = new Uint8Array(4);
    return new DataView(n.buffer).setInt32(0, t, !0), this.raw(n);
  }
  /**
   * Write a `sint32` value, a signed, zigzag-encoded 32-bit varint.
   */
  sint32(t) {
    return D(t), t = (t << 1 ^ t >> 31) >>> 0, G(t, this.buf), this;
  }
  /**
   * Write a `fixed64` value, a signed, fixed-length 64-bit integer.
   */
  sfixed64(t) {
    let n = new Uint8Array(8), b = new DataView(n.buffer), c = H.enc(t);
    return b.setInt32(0, c.lo, !0), b.setInt32(4, c.hi, !0), this.raw(n);
  }
  /**
   * Write a `fixed64` value, an unsigned, fixed-length 64 bit integer.
   */
  fixed64(t) {
    let n = new Uint8Array(8), b = new DataView(n.buffer), c = H.uEnc(t);
    return b.setInt32(0, c.lo, !0), b.setInt32(4, c.hi, !0), this.raw(n);
  }
  /**
   * Write a `int64` value, a signed 64-bit varint.
   */
  int64(t) {
    let n = H.enc(t);
    return U(n.lo, n.hi, this.buf), this;
  }
  /**
   * Write a `sint64` value, a signed, zig-zag-encoded 64-bit varint.
   */
  sint64(t) {
    const n = H.enc(t), b = n.hi >> 31, c = n.lo << 1 ^ b, r = (n.hi << 1 | n.lo >>> 31) ^ b;
    return U(c, r, this.buf), this;
  }
  /**
   * Write a `uint64` value, an unsigned 64-bit varint.
   */
  uint64(t) {
    const n = H.uEnc(t);
    return U(n.lo, n.hi, this.buf), this;
  }
}
class O {
  constructor(t, n = se().decodeUtf8) {
    this.decodeUtf8 = n, this.varint64 = pe, this.uint32 = ve, this.buf = t, this.len = t.length, this.pos = 0, this.view = new DataView(t.buffer, t.byteOffset, t.byteLength);
  }
  /**
   * Reads a tag - field number and wire type.
   */
  tag() {
    let t = this.uint32(), n = t >>> 3, b = t & 7;
    if (n <= 0 || b < 0 || b > 5)
      throw new Error("illegal tag: field no " + n + " wire type " + b);
    return [n, b];
  }
  /**
   * Skip one element and return the skipped data.
   *
   * When skipping StartGroup, provide the tags field number to check for
   * matching field number in the EndGroup tag.
   */
  skip(t, n) {
    let b = this.pos;
    switch (t) {
      case T.Varint:
        for (; this.buf[this.pos++] & 128; )
          ;
        break;
      // @ts-ignore TS7029: Fallthrough case in switch -- ignore instead of expect-error for compiler settings without noFallthroughCasesInSwitch: true
      case T.Bit64:
        this.pos += 4;
      case T.Bit32:
        this.pos += 4;
        break;
      case T.LengthDelimited:
        let c = this.uint32();
        this.pos += c;
        break;
      case T.StartGroup:
        for (; ; ) {
          const [r, s] = this.tag();
          if (s === T.EndGroup) {
            if (n !== void 0 && r !== n)
              throw new Error("invalid end group tag");
            break;
          }
          this.skip(s, r);
        }
        break;
      default:
        throw new Error("cant skip wire type " + t);
    }
    return this.assertBounds(), this.buf.subarray(b, this.pos);
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
    return H.dec(...this.varint64());
  }
  /**
   * Read a `uint64` field, an unsigned 64-bit varint.
   */
  uint64() {
    return H.uDec(...this.varint64());
  }
  /**
   * Read a `sint64` field, a signed, zig-zag-encoded 64-bit varint.
   */
  sint64() {
    let [t, n] = this.varint64(), b = -(t & 1);
    return t = (t >>> 1 | (n & 1) << 31) ^ b, n = n >>> 1 ^ b, H.dec(t, n);
  }
  /**
   * Read a `bool` field, a variant.
   */
  bool() {
    let [t, n] = this.varint64();
    return t !== 0 || n !== 0;
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
    return H.uDec(this.sfixed32(), this.sfixed32());
  }
  /**
   * Read a `fixed64` field, a signed, fixed-length 64-bit integer.
   */
  sfixed64() {
    return H.dec(this.sfixed32(), this.sfixed32());
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
    let t = this.uint32(), n = this.pos;
    return this.pos += t, this.assertBounds(), this.buf.subarray(n, n + t);
  }
  /**
   * Read a `string` field, length-delimited data converted to UTF-8 text.
   */
  string() {
    return this.decodeUtf8(this.bytes());
  }
}
function D(e) {
  if (typeof e == "string")
    e = Number(e);
  else if (typeof e != "number")
    throw new Error("invalid int32: " + typeof e);
  if (!Number.isInteger(e) || e > Ee || e < Se)
    throw new Error("invalid int32: " + e);
}
function $(e) {
  if (typeof e == "string")
    e = Number(e);
  else if (typeof e != "number")
    throw new Error("invalid uint32: " + typeof e);
  if (!Number.isInteger(e) || e > xe || e < 0)
    throw new Error("invalid uint32: " + e);
}
function ke(e) {
  if (typeof e == "string") {
    const t = e;
    if (e = Number(e), Number.isNaN(e) && t !== "NaN")
      throw new Error("invalid float32: " + t);
  } else if (typeof e != "number")
    throw new Error("invalid float32: " + typeof e);
  if (Number.isFinite(e) && (e > we || e < me))
    throw new Error("invalid float32: " + e);
}
function Z() {
  return { t: 0, x: 0, y: 0, heading: 0, vl: 0, vr: 0, omega: 0, al: 0, ar: 0, alpha: 0, fl: 0, fr: 0 };
}
const ae = {
  encode(e, t = new C()) {
    return e.t !== 0 && t.uint32(9).double(e.t), e.x !== 0 && t.uint32(17).double(e.x), e.y !== 0 && t.uint32(25).double(e.y), e.heading !== 0 && t.uint32(33).double(e.heading), e.vl !== 0 && t.uint32(41).double(e.vl), e.vr !== 0 && t.uint32(49).double(e.vr), e.omega !== 0 && t.uint32(57).double(e.omega), e.al !== 0 && t.uint32(65).double(e.al), e.ar !== 0 && t.uint32(73).double(e.ar), e.alpha !== 0 && t.uint32(81).double(e.alpha), e.fl !== 0 && t.uint32(89).double(e.fl), e.fr !== 0 && t.uint32(97).double(e.fr), t;
  },
  decode(e, t) {
    const n = e instanceof O ? e : new O(e), b = t === void 0 ? n.len : n.pos + t, c = Z();
    for (; n.pos < b; ) {
      const r = n.uint32();
      switch (r >>> 3) {
        case 1: {
          if (r !== 9)
            break;
          c.t = n.double();
          continue;
        }
        case 2: {
          if (r !== 17)
            break;
          c.x = n.double();
          continue;
        }
        case 3: {
          if (r !== 25)
            break;
          c.y = n.double();
          continue;
        }
        case 4: {
          if (r !== 33)
            break;
          c.heading = n.double();
          continue;
        }
        case 5: {
          if (r !== 41)
            break;
          c.vl = n.double();
          continue;
        }
        case 6: {
          if (r !== 49)
            break;
          c.vr = n.double();
          continue;
        }
        case 7: {
          if (r !== 57)
            break;
          c.omega = n.double();
          continue;
        }
        case 8: {
          if (r !== 65)
            break;
          c.al = n.double();
          continue;
        }
        case 9: {
          if (r !== 73)
            break;
          c.ar = n.double();
          continue;
        }
        case 10: {
          if (r !== 81)
            break;
          c.alpha = n.double();
          continue;
        }
        case 11: {
          if (r !== 89)
            break;
          c.fl = n.double();
          continue;
        }
        case 12: {
          if (r !== 97)
            break;
          c.fr = n.double();
          continue;
        }
      }
      if ((r & 7) === 4 || r === 0)
        break;
      n.skip(r & 7);
    }
    return c;
  },
  fromJSON(e) {
    return {
      t: k(e.t) ? globalThis.Number(e.t) : 0,
      x: k(e.x) ? globalThis.Number(e.x) : 0,
      y: k(e.y) ? globalThis.Number(e.y) : 0,
      heading: k(e.heading) ? globalThis.Number(e.heading) : 0,
      vl: k(e.vl) ? globalThis.Number(e.vl) : 0,
      vr: k(e.vr) ? globalThis.Number(e.vr) : 0,
      omega: k(e.omega) ? globalThis.Number(e.omega) : 0,
      al: k(e.al) ? globalThis.Number(e.al) : 0,
      ar: k(e.ar) ? globalThis.Number(e.ar) : 0,
      alpha: k(e.alpha) ? globalThis.Number(e.alpha) : 0,
      fl: k(e.fl) ? globalThis.Number(e.fl) : 0,
      fr: k(e.fr) ? globalThis.Number(e.fr) : 0
    };
  },
  toJSON(e) {
    const t = {};
    return e.t !== 0 && (t.t = e.t), e.x !== 0 && (t.x = e.x), e.y !== 0 && (t.y = e.y), e.heading !== 0 && (t.heading = e.heading), e.vl !== 0 && (t.vl = e.vl), e.vr !== 0 && (t.vr = e.vr), e.omega !== 0 && (t.omega = e.omega), e.al !== 0 && (t.al = e.al), e.ar !== 0 && (t.ar = e.ar), e.alpha !== 0 && (t.alpha = e.alpha), e.fl !== 0 && (t.fl = e.fl), e.fr !== 0 && (t.fr = e.fr), t;
  },
  create(e) {
    return ae.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Z();
    return t.t = e.t ?? 0, t.x = e.x ?? 0, t.y = e.y ?? 0, t.heading = e.heading ?? 0, t.vl = e.vl ?? 0, t.vr = e.vr ?? 0, t.omega = e.omega ?? 0, t.al = e.al ?? 0, t.ar = e.ar ?? 0, t.alpha = e.alpha ?? 0, t.fl = e.fl ?? 0, t.fr = e.fr ?? 0, t;
  }
};
function k(e) {
  return e != null;
}
function Q() {
  return { x: 0, y: 0 };
}
const x = {
  encode(e, t = new C()) {
    return e.x !== 0 && t.uint32(9).double(e.x), e.y !== 0 && t.uint32(17).double(e.y), t;
  },
  decode(e, t) {
    const n = e instanceof O ? e : new O(e), b = t === void 0 ? n.len : n.pos + t, c = Q();
    for (; n.pos < b; ) {
      const r = n.uint32();
      switch (r >>> 3) {
        case 1: {
          if (r !== 9)
            break;
          c.x = n.double();
          continue;
        }
        case 2: {
          if (r !== 17)
            break;
          c.y = n.double();
          continue;
        }
      }
      if ((r & 7) === 4 || r === 0)
        break;
      n.skip(r & 7);
    }
    return c;
  },
  fromJSON(e) {
    return {
      x: E(e.x) ? globalThis.Number(e.x) : 0,
      y: E(e.y) ? globalThis.Number(e.y) : 0
    };
  },
  toJSON(e) {
    const t = {};
    return e.x !== 0 && (t.x = e.x), e.y !== 0 && (t.y = e.y), t;
  },
  create(e) {
    return x.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Q();
    return t.x = e.x ?? 0, t.y = e.y ?? 0, t;
  }
};
function Y() {
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
const M = {
  encode(e, t = new C()) {
    return e.t !== 0 && t.uint32(9).double(e.t), e.x !== 0 && t.uint32(17).double(e.x), e.y !== 0 && t.uint32(25).double(e.y), e.heading !== 0 && t.uint32(33).double(e.heading), e.vx !== 0 && t.uint32(41).double(e.vx), e.vy !== 0 && t.uint32(49).double(e.vy), e.omega !== 0 && t.uint32(57).double(e.omega), e.ax !== 0 && t.uint32(65).double(e.ax), e.ay !== 0 && t.uint32(73).double(e.ay), e.alpha !== 0 && t.uint32(81).double(e.alpha), e.fl !== void 0 && x.encode(e.fl, t.uint32(90).fork()).join(), e.fr !== void 0 && x.encode(e.fr, t.uint32(98).fork()).join(), e.bl !== void 0 && x.encode(e.bl, t.uint32(106).fork()).join(), e.br !== void 0 && x.encode(e.br, t.uint32(114).fork()).join(), t;
  },
  decode(e, t) {
    const n = e instanceof O ? e : new O(e), b = t === void 0 ? n.len : n.pos + t, c = Y();
    for (; n.pos < b; ) {
      const r = n.uint32();
      switch (r >>> 3) {
        case 1: {
          if (r !== 9)
            break;
          c.t = n.double();
          continue;
        }
        case 2: {
          if (r !== 17)
            break;
          c.x = n.double();
          continue;
        }
        case 3: {
          if (r !== 25)
            break;
          c.y = n.double();
          continue;
        }
        case 4: {
          if (r !== 33)
            break;
          c.heading = n.double();
          continue;
        }
        case 5: {
          if (r !== 41)
            break;
          c.vx = n.double();
          continue;
        }
        case 6: {
          if (r !== 49)
            break;
          c.vy = n.double();
          continue;
        }
        case 7: {
          if (r !== 57)
            break;
          c.omega = n.double();
          continue;
        }
        case 8: {
          if (r !== 65)
            break;
          c.ax = n.double();
          continue;
        }
        case 9: {
          if (r !== 73)
            break;
          c.ay = n.double();
          continue;
        }
        case 10: {
          if (r !== 81)
            break;
          c.alpha = n.double();
          continue;
        }
        case 11: {
          if (r !== 90)
            break;
          c.fl = x.decode(n, n.uint32());
          continue;
        }
        case 12: {
          if (r !== 98)
            break;
          c.fr = x.decode(n, n.uint32());
          continue;
        }
        case 13: {
          if (r !== 106)
            break;
          c.bl = x.decode(n, n.uint32());
          continue;
        }
        case 14: {
          if (r !== 114)
            break;
          c.br = x.decode(n, n.uint32());
          continue;
        }
      }
      if ((r & 7) === 4 || r === 0)
        break;
      n.skip(r & 7);
    }
    return c;
  },
  fromJSON(e) {
    return {
      t: E(e.t) ? globalThis.Number(e.t) : 0,
      x: E(e.x) ? globalThis.Number(e.x) : 0,
      y: E(e.y) ? globalThis.Number(e.y) : 0,
      heading: E(e.heading) ? globalThis.Number(e.heading) : 0,
      vx: E(e.vx) ? globalThis.Number(e.vx) : 0,
      vy: E(e.vy) ? globalThis.Number(e.vy) : 0,
      omega: E(e.omega) ? globalThis.Number(e.omega) : 0,
      ax: E(e.ax) ? globalThis.Number(e.ax) : 0,
      ay: E(e.ay) ? globalThis.Number(e.ay) : 0,
      alpha: E(e.alpha) ? globalThis.Number(e.alpha) : 0,
      fl: E(e.fl) ? x.fromJSON(e.fl) : void 0,
      fr: E(e.fr) ? x.fromJSON(e.fr) : void 0,
      bl: E(e.bl) ? x.fromJSON(e.bl) : void 0,
      br: E(e.br) ? x.fromJSON(e.br) : void 0
    };
  },
  toJSON(e) {
    const t = {};
    return e.t !== 0 && (t.t = e.t), e.x !== 0 && (t.x = e.x), e.y !== 0 && (t.y = e.y), e.heading !== 0 && (t.heading = e.heading), e.vx !== 0 && (t.vx = e.vx), e.vy !== 0 && (t.vy = e.vy), e.omega !== 0 && (t.omega = e.omega), e.ax !== 0 && (t.ax = e.ax), e.ay !== 0 && (t.ay = e.ay), e.alpha !== 0 && (t.alpha = e.alpha), e.fl !== void 0 && (t.fl = x.toJSON(e.fl)), e.fr !== void 0 && (t.fr = x.toJSON(e.fr)), e.bl !== void 0 && (t.bl = x.toJSON(e.bl)), e.br !== void 0 && (t.br = x.toJSON(e.br)), t;
  },
  create(e) {
    return M.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = Y();
    return t.t = e.t ?? 0, t.x = e.x ?? 0, t.y = e.y ?? 0, t.heading = e.heading ?? 0, t.vx = e.vx ?? 0, t.vy = e.vy ?? 0, t.omega = e.omega ?? 0, t.ax = e.ax ?? 0, t.ay = e.ay ?? 0, t.alpha = e.alpha ?? 0, t.fl = e.fl !== void 0 && e.fl !== null ? x.fromPartial(e.fl) : void 0, t.fr = e.fr !== void 0 && e.fr !== null ? x.fromPartial(e.fr) : void 0, t.bl = e.bl !== void 0 && e.bl !== null ? x.fromPartial(e.bl) : void 0, t.br = e.br !== void 0 && e.br !== null ? x.fromPartial(e.br) : void 0, t;
  }
};
function E(e) {
  return e != null;
}
const Ie = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  DifferentialSample: ae,
  ForceVector: x,
  SwerveSample: M
}, Symbol.toStringTag, { value: "Module" }));
function j() {
  return { sample: void 0 };
}
const P = {
  encode(e, t = new C()) {
    return e.sample !== void 0 && M.encode(e.sample, t.uint32(10).fork()).join(), t;
  },
  decode(e, t) {
    const n = e instanceof O ? e : new O(e), b = t === void 0 ? n.len : n.pos + t, c = j();
    for (; n.pos < b; ) {
      const r = n.uint32();
      switch (r >>> 3) {
        case 1: {
          if (r !== 10)
            break;
          c.sample = M.decode(n, n.uint32());
          continue;
        }
      }
      if ((r & 7) === 4 || r === 0)
        break;
      n.skip(r & 7);
    }
    return c;
  },
  fromJSON(e) {
    return { sample: ue(e.sample) ? M.fromJSON(e.sample) : void 0 };
  },
  toJSON(e) {
    const t = {};
    return e.sample !== void 0 && (t.sample = M.toJSON(e.sample)), t;
  },
  create(e) {
    return P.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = j();
    return t.sample = e.sample !== void 0 && e.sample !== null ? M.fromPartial(e.sample) : void 0, t;
  }
};
function ee() {
  return { sample: void 0 };
}
const V = {
  encode(e, t = new C()) {
    return e.sample !== void 0 && M.encode(e.sample, t.uint32(10).fork()).join(), t;
  },
  decode(e, t) {
    const n = e instanceof O ? e : new O(e), b = t === void 0 ? n.len : n.pos + t, c = ee();
    for (; n.pos < b; ) {
      const r = n.uint32();
      switch (r >>> 3) {
        case 1: {
          if (r !== 10)
            break;
          c.sample = M.decode(n, n.uint32());
          continue;
        }
      }
      if ((r & 7) === 4 || r === 0)
        break;
      n.skip(r & 7);
    }
    return c;
  },
  fromJSON(e) {
    return { sample: ue(e.sample) ? M.fromJSON(e.sample) : void 0 };
  },
  toJSON(e) {
    const t = {};
    return e.sample !== void 0 && (t.sample = M.toJSON(e.sample)), t;
  },
  create(e) {
    return V.fromPartial(e ?? {});
  },
  fromPartial(e) {
    const t = ee();
    return t.sample = e.sample !== void 0 && e.sample !== null ? M.fromPartial(e.sample) : void 0, t;
  }
};
function ue(e) {
  return e != null;
}
const Me = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  EchoSwerveSampleRequest: P,
  EchoSwerveSampleResponse: V
}, Symbol.toStringTag, { value: "Module" }));
var A = { exports: {} }, He = A.exports, te;
function Oe() {
  return te || (te = 1, (function(e, t) {
    (function(n, b) {
      e.exports = b();
    })(He, (function() {
      return n = { 418: function(c, r) {
        (function(s, o) {
          for (var y in o) s[y] = o[y];
        })(r, (function(s) {
          var o = {};
          function y(a) {
            if (o[a]) return o[a].exports;
            var f = o[a] = { i: a, l: !1, exports: {} };
            return s[a].call(f.exports, f, f.exports, y), f.l = !0, f.exports;
          }
          return y.m = s, y.c = o, y.i = function(a) {
            return a;
          }, y.d = function(a, f, v) {
            y.o(a, f) || Object.defineProperty(a, f, { configurable: !1, enumerable: !0, get: v });
          }, y.n = function(a) {
            var f = a && a.__esModule ? function() {
              return a.default;
            } : function() {
              return a;
            };
            return y.d(f, "a", f), f;
          }, y.o = function(a, f) {
            return Object.prototype.hasOwnProperty.call(a, f);
          }, y.p = "", y(y.s = 1);
        })([function(s, o, y) {
          Object.defineProperty(o, "__esModule", { value: !0 });
          var a = y(3), f = (function() {
            function v(u, h) {
              u === void 0 && (u = {}), h === void 0 && (h = { splitValues: !1 });
              var l, p = this;
              this.headersMap = {}, u && (typeof Headers < "u" && u instanceof Headers ? a.getHeaderKeys(u).forEach((function(i) {
                a.getHeaderValues(u, i).forEach((function(d) {
                  h.splitValues ? p.append(i, a.splitHeaderValue(d)) : p.append(i, d);
                }));
              })) : typeof (l = u) == "object" && typeof l.headersMap == "object" && typeof l.forEach == "function" ? u.forEach((function(i, d) {
                p.append(i, d);
              })) : typeof Map < "u" && u instanceof Map ? u.forEach((function(i, d) {
                p.append(d, i);
              })) : typeof u == "string" ? this.appendFromString(u) : typeof u == "object" && Object.getOwnPropertyNames(u).forEach((function(i) {
                var d = u[i];
                Array.isArray(d) ? d.forEach((function(g) {
                  p.append(i, g);
                })) : p.append(i, d);
              })));
            }
            return v.prototype.appendFromString = function(u) {
              for (var h = u.split(`\r
`), l = 0; l < h.length; l++) {
                var p = h[l], i = p.indexOf(":");
                if (i > 0) {
                  var d = p.substring(0, i).trim(), g = p.substring(i + 1).trim();
                  this.append(d, g);
                }
              }
            }, v.prototype.delete = function(u, h) {
              var l = a.normalizeName(u);
              if (h === void 0) delete this.headersMap[l];
              else {
                var p = this.headersMap[l];
                if (p) {
                  var i = p.indexOf(h);
                  i >= 0 && p.splice(i, 1), p.length === 0 && delete this.headersMap[l];
                }
              }
            }, v.prototype.append = function(u, h) {
              var l = this, p = a.normalizeName(u);
              Array.isArray(this.headersMap[p]) || (this.headersMap[p] = []), Array.isArray(h) ? h.forEach((function(i) {
                l.headersMap[p].push(a.normalizeValue(i));
              })) : this.headersMap[p].push(a.normalizeValue(h));
            }, v.prototype.set = function(u, h) {
              var l = a.normalizeName(u);
              if (Array.isArray(h)) {
                var p = [];
                h.forEach((function(i) {
                  p.push(a.normalizeValue(i));
                })), this.headersMap[l] = p;
              } else this.headersMap[l] = [a.normalizeValue(h)];
            }, v.prototype.has = function(u, h) {
              var l = this.headersMap[a.normalizeName(u)];
              if (!Array.isArray(l)) return !1;
              if (h !== void 0) {
                var p = a.normalizeValue(h);
                return l.indexOf(p) >= 0;
              }
              return !0;
            }, v.prototype.get = function(u) {
              var h = this.headersMap[a.normalizeName(u)];
              return h !== void 0 ? h.concat() : [];
            }, v.prototype.forEach = function(u) {
              var h = this;
              Object.getOwnPropertyNames(this.headersMap).forEach((function(l) {
                u(l, h.headersMap[l]);
              }), this);
            }, v.prototype.toHeaders = function() {
              if (typeof Headers < "u") {
                var u = new Headers();
                return this.forEach((function(h, l) {
                  l.forEach((function(p) {
                    u.append(h, p);
                  }));
                })), u;
              }
              throw new Error("Headers class is not defined");
            }, v;
          })();
          o.BrowserHeaders = f;
        }, function(s, o, y) {
          Object.defineProperty(o, "__esModule", { value: !0 });
          var a = y(0);
          o.BrowserHeaders = a.BrowserHeaders;
        }, function(s, o, y) {
          Object.defineProperty(o, "__esModule", { value: !0 }), o.iterateHeaders = function(a, f) {
            for (var v = a[Symbol.iterator](), u = v.next(); !u.done; ) f(u.value[0]), u = v.next();
          }, o.iterateHeadersKeys = function(a, f) {
            for (var v = a.keys(), u = v.next(); !u.done; ) f(u.value), u = v.next();
          };
        }, function(s, o, y) {
          Object.defineProperty(o, "__esModule", { value: !0 });
          var a = y(2);
          o.normalizeName = function(f) {
            if (typeof f != "string" && (f = String(f)), /[^a-z0-9\-#$%&'*+.\^_`|~]/i.test(f)) throw new TypeError("Invalid character in header field name");
            return f.toLowerCase();
          }, o.normalizeValue = function(f) {
            return typeof f != "string" && (f = String(f)), f;
          }, o.getHeaderValues = function(f, v) {
            var u = f;
            if (u instanceof Headers && u.getAll) return u.getAll(v);
            var h = u.get(v);
            return h && typeof h == "string" ? [h] : h;
          }, o.getHeaderKeys = function(f) {
            var v = f, u = {}, h = [];
            return v.keys ? a.iterateHeadersKeys(v, (function(l) {
              u[l] || (u[l] = !0, h.push(l));
            })) : v.forEach ? v.forEach((function(l, p) {
              u[p] || (u[p] = !0, h.push(p));
            })) : a.iterateHeaders(v, (function(l) {
              var p = l[0];
              u[p] || (u[p] = !0, h.push(p));
            })), h;
          }, o.splitHeaderValue = function(f) {
            var v = [];
            return f.split(", ").forEach((function(u) {
              u.split(",").forEach((function(h) {
                v.push(h);
              }));
            })), v;
          };
        }]));
      }, 617: function(c, r, s) {
        Object.defineProperty(r, "__esModule", { value: !0 }), r.ChunkParser = r.ChunkType = r.encodeASCII = r.decodeASCII = void 0;
        var o, y = s(65);
        function a(i) {
          return (d = i) === 9 || d === 10 || d === 13 || i >= 32 && i <= 126;
          var d;
        }
        function f(i) {
          for (var d = 0; d !== i.length; ++d) if (!a(i[d])) throw new Error("Metadata is not valid (printable) ASCII");
          return String.fromCharCode.apply(String, Array.prototype.slice.call(i));
        }
        function v(i) {
          return (128 & i.getUint8(0)) == 128;
        }
        function u(i) {
          return i.getUint32(1, !1);
        }
        function h(i, d, g) {
          return i.byteLength - d >= g;
        }
        function l(i, d, g) {
          if (i.slice) return i.slice(d, g);
          var w = i.length;
          g !== void 0 && (w = g);
          for (var m = new Uint8Array(w - d), S = 0, _ = d; _ < w; _++) m[S++] = i[_];
          return m;
        }
        r.decodeASCII = f, r.encodeASCII = function(i) {
          for (var d = new Uint8Array(i.length), g = 0; g !== i.length; ++g) {
            var w = i.charCodeAt(g);
            if (!a(w)) throw new Error("Metadata contains invalid ASCII");
            d[g] = w;
          }
          return d;
        }, (function(i) {
          i[i.MESSAGE = 1] = "MESSAGE", i[i.TRAILERS = 2] = "TRAILERS";
        })(o = r.ChunkType || (r.ChunkType = {}));
        var p = (function() {
          function i() {
            this.buffer = null, this.position = 0;
          }
          return i.prototype.parse = function(d, g) {
            if (d.length === 0 && g) return [];
            var w, m = [];
            if (this.buffer == null) this.buffer = d, this.position = 0;
            else if (this.position === this.buffer.byteLength) this.buffer = d, this.position = 0;
            else {
              var S = this.buffer.byteLength - this.position, _ = new Uint8Array(S + d.byteLength), he = l(this.buffer, this.position);
              _.set(he, 0);
              var le = new Uint8Array(d);
              _.set(le, S), this.buffer = _, this.position = 0;
            }
            for (; ; ) {
              if (!h(this.buffer, this.position, 5)) return m;
              var R = l(this.buffer, this.position, this.position + 5), L = new DataView(R.buffer, R.byteOffset, R.byteLength), B = u(L);
              if (!h(this.buffer, this.position, 5 + B)) return m;
              var q = l(this.buffer, this.position + 5, this.position + 5 + B);
              if (this.position += 5 + B, v(L)) return m.push({ chunkType: o.TRAILERS, trailers: (w = q, new y.Metadata(f(w))) }), m;
              m.push({ chunkType: o.MESSAGE, data: q });
            }
          }, i;
        })();
        r.ChunkParser = p;
      }, 8: function(c, r) {
        var s;
        Object.defineProperty(r, "__esModule", { value: !0 }), r.httpStatusToCode = r.Code = void 0, (function(o) {
          o[o.OK = 0] = "OK", o[o.Canceled = 1] = "Canceled", o[o.Unknown = 2] = "Unknown", o[o.InvalidArgument = 3] = "InvalidArgument", o[o.DeadlineExceeded = 4] = "DeadlineExceeded", o[o.NotFound = 5] = "NotFound", o[o.AlreadyExists = 6] = "AlreadyExists", o[o.PermissionDenied = 7] = "PermissionDenied", o[o.ResourceExhausted = 8] = "ResourceExhausted", o[o.FailedPrecondition = 9] = "FailedPrecondition", o[o.Aborted = 10] = "Aborted", o[o.OutOfRange = 11] = "OutOfRange", o[o.Unimplemented = 12] = "Unimplemented", o[o.Internal = 13] = "Internal", o[o.Unavailable = 14] = "Unavailable", o[o.DataLoss = 15] = "DataLoss", o[o.Unauthenticated = 16] = "Unauthenticated";
        })(s = r.Code || (r.Code = {})), r.httpStatusToCode = function(o) {
          switch (o) {
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
      }, 934: function(c, r, s) {
        Object.defineProperty(r, "__esModule", { value: !0 }), r.client = void 0;
        var o = s(65), y = s(617), a = s(8), f = s(346), v = s(57), u = s(882);
        r.client = function(p, i) {
          return new h(p, i);
        };
        var h = (function() {
          function p(i, d) {
            this.started = !1, this.sentFirstMessage = !1, this.completed = !1, this.closed = !1, this.finishedSending = !1, this.onHeadersCallbacks = [], this.onMessageCallbacks = [], this.onEndCallbacks = [], this.parser = new y.ChunkParser(), this.methodDefinition = i, this.props = d, this.createTransport();
          }
          return p.prototype.createTransport = function() {
            var i = this.props.host + "/" + this.methodDefinition.service.serviceName + "/" + this.methodDefinition.methodName, d = { methodDefinition: this.methodDefinition, debug: this.props.debug || !1, url: i, onHeaders: this.onTransportHeaders.bind(this), onChunk: this.onTransportChunk.bind(this), onEnd: this.onTransportEnd.bind(this) };
            this.props.transport ? this.transport = this.props.transport(d) : this.transport = v.makeDefaultTransport(d);
          }, p.prototype.onTransportHeaders = function(i, d) {
            if (this.props.debug && f.debug("onHeaders", i, d), this.closed) this.props.debug && f.debug("grpc.onHeaders received after request was closed - ignoring");
            else if (d !== 0) {
              this.responseHeaders = i, this.props.debug && f.debug("onHeaders.responseHeaders", JSON.stringify(this.responseHeaders, null, 2));
              var g = l(i);
              this.props.debug && f.debug("onHeaders.gRPCStatus", g);
              var w = g && g >= 0 ? g : a.httpStatusToCode(d);
              this.props.debug && f.debug("onHeaders.code", w);
              var m = i.get("grpc-message") || [];
              if (this.props.debug && f.debug("onHeaders.gRPCMessage", m), this.rawOnHeaders(i), w !== a.Code.OK) {
                var S = this.decodeGRPCStatus(m[0]);
                this.rawOnError(w, S, i);
              }
            }
          }, p.prototype.onTransportChunk = function(i) {
            var d = this;
            if (this.closed) this.props.debug && f.debug("grpc.onChunk received after request was closed - ignoring");
            else {
              var g = [];
              try {
                g = this.parser.parse(i);
              } catch (w) {
                return this.props.debug && f.debug("onChunk.parsing error", w, w.message), void this.rawOnError(a.Code.Internal, "parsing error: " + w.message);
              }
              g.forEach((function(w) {
                if (w.chunkType === y.ChunkType.MESSAGE) {
                  var m = d.methodDefinition.responseType.deserializeBinary(w.data);
                  d.rawOnMessage(m);
                } else w.chunkType === y.ChunkType.TRAILERS && (d.responseHeaders ? (d.responseTrailers = new o.Metadata(w.trailers), d.props.debug && f.debug("onChunk.trailers", d.responseTrailers)) : (d.responseHeaders = new o.Metadata(w.trailers), d.rawOnHeaders(d.responseHeaders)));
              }));
            }
          }, p.prototype.onTransportEnd = function() {
            if (this.props.debug && f.debug("grpc.onEnd"), this.closed) this.props.debug && f.debug("grpc.onEnd received after request was closed - ignoring");
            else if (this.responseTrailers !== void 0) {
              var i = l(this.responseTrailers);
              if (i !== null) {
                var d = this.responseTrailers.get("grpc-message"), g = this.decodeGRPCStatus(d[0]);
                this.rawOnEnd(i, g, this.responseTrailers);
              } else this.rawOnError(a.Code.Internal, "Response closed without grpc-status (Trailers provided)");
            } else {
              if (this.responseHeaders === void 0) return void this.rawOnError(a.Code.Unknown, "Response closed without headers");
              var w = l(this.responseHeaders), m = this.responseHeaders.get("grpc-message");
              if (this.props.debug && f.debug("grpc.headers only response ", w, m), w === null) return void this.rawOnEnd(a.Code.Unknown, "Response closed without grpc-status (Headers only)", this.responseHeaders);
              var S = this.decodeGRPCStatus(m[0]);
              this.rawOnEnd(w, S, this.responseHeaders);
            }
          }, p.prototype.decodeGRPCStatus = function(i) {
            if (!i) return "";
            try {
              return decodeURIComponent(i);
            } catch {
              return i;
            }
          }, p.prototype.rawOnEnd = function(i, d, g) {
            var w = this;
            this.props.debug && f.debug("rawOnEnd", i, d, g), this.completed || (this.completed = !0, this.onEndCallbacks.forEach((function(m) {
              if (!w.closed) try {
                m(i, d, g);
              } catch (S) {
                setTimeout((function() {
                  throw S;
                }), 0);
              }
            })));
          }, p.prototype.rawOnHeaders = function(i) {
            this.props.debug && f.debug("rawOnHeaders", i), this.completed || this.onHeadersCallbacks.forEach((function(d) {
              try {
                d(i);
              } catch (g) {
                setTimeout((function() {
                  throw g;
                }), 0);
              }
            }));
          }, p.prototype.rawOnError = function(i, d, g) {
            var w = this;
            g === void 0 && (g = new o.Metadata()), this.props.debug && f.debug("rawOnError", i, d), this.completed || (this.completed = !0, this.onEndCallbacks.forEach((function(m) {
              if (!w.closed) try {
                m(i, d, g);
              } catch (S) {
                setTimeout((function() {
                  throw S;
                }), 0);
              }
            })));
          }, p.prototype.rawOnMessage = function(i) {
            var d = this;
            this.props.debug && f.debug("rawOnMessage", i.toObject()), this.completed || this.closed || this.onMessageCallbacks.forEach((function(g) {
              if (!d.closed) try {
                g(i);
              } catch (w) {
                setTimeout((function() {
                  throw w;
                }), 0);
              }
            }));
          }, p.prototype.onHeaders = function(i) {
            this.onHeadersCallbacks.push(i);
          }, p.prototype.onMessage = function(i) {
            this.onMessageCallbacks.push(i);
          }, p.prototype.onEnd = function(i) {
            this.onEndCallbacks.push(i);
          }, p.prototype.start = function(i) {
            if (this.started) throw new Error("Client already started - cannot .start()");
            this.started = !0;
            var d = new o.Metadata(i || {});
            d.set("content-type", "application/grpc-web+proto"), d.set("x-grpc-web", "1"), this.transport.start(d);
          }, p.prototype.send = function(i) {
            if (!this.started) throw new Error("Client not started - .start() must be called before .send()");
            if (this.closed) throw new Error("Client already closed - cannot .send()");
            if (this.finishedSending) throw new Error("Client already finished sending - cannot .send()");
            if (!this.methodDefinition.requestStream && this.sentFirstMessage) throw new Error("Message already sent for non-client-streaming method - cannot .send()");
            this.sentFirstMessage = !0;
            var d = u.frameRequest(i);
            this.transport.sendMessage(d);
          }, p.prototype.finishSend = function() {
            if (!this.started) throw new Error("Client not started - .finishSend() must be called before .close()");
            if (this.closed) throw new Error("Client already closed - cannot .send()");
            if (this.finishedSending) throw new Error("Client already finished sending - cannot .finishSend()");
            this.finishedSending = !0, this.transport.finishSend();
          }, p.prototype.close = function() {
            if (!this.started) throw new Error("Client not started - .start() must be called before .close()");
            if (this.closed) throw new Error("Client already closed - cannot .close()");
            this.closed = !0, this.props.debug && f.debug("request.abort aborting request"), this.transport.cancel();
          }, p;
        })();
        function l(p) {
          var i = p.get("grpc-status") || [];
          if (i.length > 0) try {
            var d = i[0];
            return parseInt(d, 10);
          } catch {
            return null;
          }
          return null;
        }
      }, 346: function(c, r) {
        Object.defineProperty(r, "__esModule", { value: !0 }), r.debug = void 0, r.debug = function() {
          for (var s = [], o = 0; o < arguments.length; o++) s[o] = arguments[o];
          console.debug ? console.debug.apply(null, s) : console.log.apply(null, s);
        };
      }, 607: function(c, r, s) {
        Object.defineProperty(r, "__esModule", { value: !0 }), r.grpc = void 0;
        var o, y = s(418), a = s(57), f = s(229), v = s(540), u = s(210), h = s(859), l = s(8), p = s(938), i = s(35), d = s(934);
        (o = r.grpc || (r.grpc = {})).setDefaultTransport = a.setDefaultTransportFactory, o.CrossBrowserHttpTransport = h.CrossBrowserHttpTransport, o.FetchReadableStreamTransport = f.FetchReadableStreamTransport, o.XhrTransport = u.XhrTransport, o.WebsocketTransport = v.WebsocketTransport, o.Code = l.Code, o.Metadata = y.BrowserHeaders, o.client = function(g, w) {
          return d.client(g, w);
        }, o.invoke = p.invoke, o.unary = i.unary;
      }, 938: function(c, r, s) {
        Object.defineProperty(r, "__esModule", { value: !0 }), r.invoke = void 0;
        var o = s(934);
        r.invoke = function(y, a) {
          if (y.requestStream) throw new Error(".invoke cannot be used with client-streaming methods. Use .client instead.");
          var f = o.client(y, { host: a.host, transport: a.transport, debug: a.debug });
          return a.onHeaders && f.onHeaders(a.onHeaders), a.onMessage && f.onMessage(a.onMessage), a.onEnd && f.onEnd(a.onEnd), f.start(a.metadata), f.send(a.request), f.finishSend(), { close: function() {
            f.close();
          } };
        };
      }, 65: function(c, r, s) {
        Object.defineProperty(r, "__esModule", { value: !0 }), r.Metadata = void 0;
        var o = s(418);
        Object.defineProperty(r, "Metadata", { enumerable: !0, get: function() {
          return o.BrowserHeaders;
        } });
      }, 57: function(c, r, s) {
        Object.defineProperty(r, "__esModule", { value: !0 }), r.makeDefaultTransport = r.setDefaultTransportFactory = void 0;
        var o = s(859), y = function(a) {
          return o.CrossBrowserHttpTransport({ withCredentials: !1 })(a);
        };
        r.setDefaultTransportFactory = function(a) {
          y = a;
        }, r.makeDefaultTransport = function(a) {
          return y(a);
        };
      }, 229: function(c, r, s) {
        var o = this && this.__assign || function() {
          return (o = Object.assign || function(v) {
            for (var u, h = 1, l = arguments.length; h < l; h++) for (var p in u = arguments[h]) Object.prototype.hasOwnProperty.call(u, p) && (v[p] = u[p]);
            return v;
          }).apply(this, arguments);
        };
        Object.defineProperty(r, "__esModule", { value: !0 }), r.detectFetchSupport = r.FetchReadableStreamTransport = void 0;
        var y = s(65), a = s(346);
        r.FetchReadableStreamTransport = function(v) {
          return function(u) {
            return (function(h, l) {
              return h.debug && a.debug("fetchRequest", h), new f(h, l);
            })(u, v);
          };
        };
        var f = (function() {
          function v(u, h) {
            this.cancelled = !1, this.controller = self.AbortController && new AbortController(), this.options = u, this.init = h;
          }
          return v.prototype.pump = function(u, h) {
            var l = this;
            if (this.reader = u, this.cancelled) return this.options.debug && a.debug("Fetch.pump.cancel at first pump"), void this.reader.cancel().catch((function(p) {
              l.options.debug && a.debug("Fetch.pump.reader.cancel exception", p);
            }));
            this.reader.read().then((function(p) {
              if (p.done) return l.options.onEnd(), h;
              l.options.onChunk(p.value), l.pump(l.reader, h);
            })).catch((function(p) {
              l.cancelled ? l.options.debug && a.debug("Fetch.catch - request cancelled") : (l.cancelled = !0, l.options.debug && a.debug("Fetch.catch", p.message), l.options.onEnd(p));
            }));
          }, v.prototype.send = function(u) {
            var h = this;
            fetch(this.options.url, o(o({}, this.init), { headers: this.metadata.toHeaders(), method: "POST", body: u, signal: this.controller && this.controller.signal })).then((function(l) {
              if (h.options.debug && a.debug("Fetch.response", l), h.options.onHeaders(new y.Metadata(l.headers), l.status), !l.body) return l;
              h.pump(l.body.getReader(), l);
            })).catch((function(l) {
              h.cancelled ? h.options.debug && a.debug("Fetch.catch - request cancelled") : (h.cancelled = !0, h.options.debug && a.debug("Fetch.catch", l.message), h.options.onEnd(l));
            }));
          }, v.prototype.sendMessage = function(u) {
            this.send(u);
          }, v.prototype.finishSend = function() {
          }, v.prototype.start = function(u) {
            this.metadata = u;
          }, v.prototype.cancel = function() {
            var u = this;
            this.cancelled ? this.options.debug && a.debug("Fetch.cancel already cancelled") : (this.cancelled = !0, this.controller ? (this.options.debug && a.debug("Fetch.cancel.controller.abort"), this.controller.abort()) : this.options.debug && a.debug("Fetch.cancel.missing abort controller"), this.reader ? (this.options.debug && a.debug("Fetch.cancel.reader.cancel"), this.reader.cancel().catch((function(h) {
              u.options.debug && a.debug("Fetch.cancel.reader.cancel exception", h);
            }))) : this.options.debug && a.debug("Fetch.cancel before reader"));
          }, v;
        })();
        r.detectFetchSupport = function() {
          return typeof Response < "u" && Response.prototype.hasOwnProperty("body") && typeof Headers == "function";
        };
      }, 859: function(c, r, s) {
        Object.defineProperty(r, "__esModule", { value: !0 }), r.CrossBrowserHttpTransport = void 0;
        var o = s(229), y = s(210);
        r.CrossBrowserHttpTransport = function(a) {
          if (o.detectFetchSupport()) {
            var f = { credentials: a.withCredentials ? "include" : "same-origin" };
            return o.FetchReadableStreamTransport(f);
          }
          return y.XhrTransport({ withCredentials: a.withCredentials });
        };
      }, 210: function(c, r, s) {
        var o, y = this && this.__extends || (o = function(i, d) {
          return (o = Object.setPrototypeOf || { __proto__: [] } instanceof Array && function(g, w) {
            g.__proto__ = w;
          } || function(g, w) {
            for (var m in w) Object.prototype.hasOwnProperty.call(w, m) && (g[m] = w[m]);
          })(i, d);
        }, function(i, d) {
          function g() {
            this.constructor = i;
          }
          o(i, d), i.prototype = d === null ? Object.create(d) : (g.prototype = d.prototype, new g());
        });
        Object.defineProperty(r, "__esModule", { value: !0 }), r.stringToArrayBuffer = r.MozChunkedArrayBufferXHR = r.XHR = r.XhrTransport = void 0;
        var a = s(65), f = s(346), v = s(849);
        r.XhrTransport = function(i) {
          return function(d) {
            if (v.detectMozXHRSupport()) return new h(d, i);
            if (v.detectXHROverrideMimeTypeSupport()) return new u(d, i);
            throw new Error("This environment's XHR implementation cannot support binary transfer.");
          };
        };
        var u = (function() {
          function i(d, g) {
            this.options = d, this.init = g;
          }
          return i.prototype.onProgressEvent = function() {
            this.options.debug && f.debug("XHR.onProgressEvent.length: ", this.xhr.response.length);
            var d = this.xhr.response.substr(this.index);
            this.index = this.xhr.response.length;
            var g = p(d);
            this.options.onChunk(g);
          }, i.prototype.onLoadEvent = function() {
            this.options.debug && f.debug("XHR.onLoadEvent"), this.options.onEnd();
          }, i.prototype.onStateChange = function() {
            this.options.debug && f.debug("XHR.onStateChange", this.xhr.readyState), this.xhr.readyState === XMLHttpRequest.HEADERS_RECEIVED && this.options.onHeaders(new a.Metadata(this.xhr.getAllResponseHeaders()), this.xhr.status);
          }, i.prototype.sendMessage = function(d) {
            this.xhr.send(d);
          }, i.prototype.finishSend = function() {
          }, i.prototype.start = function(d) {
            var g = this;
            this.metadata = d;
            var w = new XMLHttpRequest();
            this.xhr = w, w.open("POST", this.options.url), this.configureXhr(), this.metadata.forEach((function(m, S) {
              w.setRequestHeader(m, S.join(", "));
            })), w.withCredentials = !!this.init.withCredentials, w.addEventListener("readystatechange", this.onStateChange.bind(this)), w.addEventListener("progress", this.onProgressEvent.bind(this)), w.addEventListener("loadend", this.onLoadEvent.bind(this)), w.addEventListener("error", (function(m) {
              g.options.debug && f.debug("XHR.error", m), g.options.onEnd(m.error);
            }));
          }, i.prototype.configureXhr = function() {
            this.xhr.responseType = "text", this.xhr.overrideMimeType("text/plain; charset=x-user-defined");
          }, i.prototype.cancel = function() {
            this.options.debug && f.debug("XHR.abort"), this.xhr.abort();
          }, i;
        })();
        r.XHR = u;
        var h = (function(i) {
          function d() {
            return i !== null && i.apply(this, arguments) || this;
          }
          return y(d, i), d.prototype.configureXhr = function() {
            this.options.debug && f.debug("MozXHR.configureXhr: setting responseType to 'moz-chunked-arraybuffer'"), this.xhr.responseType = "moz-chunked-arraybuffer";
          }, d.prototype.onProgressEvent = function() {
            var g = this.xhr.response;
            this.options.debug && f.debug("MozXHR.onProgressEvent: ", new Uint8Array(g)), this.options.onChunk(new Uint8Array(g));
          }, d;
        })(u);
        function l(i, d) {
          var g = i.charCodeAt(d);
          if (g >= 55296 && g <= 56319) {
            var w = i.charCodeAt(d + 1);
            w >= 56320 && w <= 57343 && (g = 65536 + (g - 55296 << 10) + (w - 56320));
          }
          return g;
        }
        function p(i) {
          for (var d = new Uint8Array(i.length), g = 0, w = 0; w < i.length; w++) {
            var m = String.prototype.codePointAt ? i.codePointAt(w) : l(i, w);
            d[g++] = 255 & m;
          }
          return d;
        }
        r.MozChunkedArrayBufferXHR = h, r.stringToArrayBuffer = p;
      }, 849: function(c, r) {
        var s;
        function o() {
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
        function y(a) {
          var f = o();
          if (!f) return !1;
          try {
            return f.responseType = a, f.responseType === a;
          } catch {
          }
          return !1;
        }
        Object.defineProperty(r, "__esModule", { value: !0 }), r.detectXHROverrideMimeTypeSupport = r.detectMozXHRSupport = r.xhrSupportsResponseType = void 0, r.xhrSupportsResponseType = y, r.detectMozXHRSupport = function() {
          return typeof XMLHttpRequest < "u" && y("moz-chunked-arraybuffer");
        }, r.detectXHROverrideMimeTypeSupport = function() {
          return typeof XMLHttpRequest < "u" && XMLHttpRequest.prototype.hasOwnProperty("overrideMimeType");
        };
      }, 540: function(c, r, s) {
        Object.defineProperty(r, "__esModule", { value: !0 }), r.WebsocketTransport = void 0;
        var o, y = s(346), a = s(617);
        (function(v) {
          v[v.FINISH_SEND = 1] = "FINISH_SEND";
        })(o || (o = {}));
        var f = new Uint8Array([1]);
        r.WebsocketTransport = function() {
          return function(v) {
            return (function(u) {
              u.debug && y.debug("websocketRequest", u);
              var h, l = (function(d) {
                if (d.substr(0, 8) === "https://") return "wss://" + d.substr(8);
                if (d.substr(0, 7) === "http://") return "ws://" + d.substr(7);
                throw new Error("Websocket transport constructed with non-https:// or http:// host.");
              })(u.url), p = [];
              function i(d) {
                if (d === o.FINISH_SEND) h.send(f);
                else {
                  var g = d, w = new Int8Array(g.byteLength + 1);
                  w.set(new Uint8Array([0])), w.set(g, 1), h.send(w);
                }
              }
              return { sendMessage: function(d) {
                h && h.readyState !== h.CONNECTING ? i(d) : p.push(d);
              }, finishSend: function() {
                h && h.readyState !== h.CONNECTING ? i(o.FINISH_SEND) : p.push(o.FINISH_SEND);
              }, start: function(d) {
                (h = new WebSocket(l, ["grpc-websockets"])).binaryType = "arraybuffer", h.onopen = function() {
                  var g;
                  u.debug && y.debug("websocketRequest.onopen"), h.send((g = "", d.forEach((function(w, m) {
                    g += w + ": " + m.join(", ") + `\r
`;
                  })), a.encodeASCII(g))), p.forEach((function(w) {
                    i(w);
                  }));
                }, h.onclose = function(g) {
                  u.debug && y.debug("websocketRequest.onclose", g), u.onEnd();
                }, h.onerror = function(g) {
                  u.debug && y.debug("websocketRequest.onerror", g);
                }, h.onmessage = function(g) {
                  u.onChunk(new Uint8Array(g.data));
                };
              }, cancel: function() {
                u.debug && y.debug("websocket.abort"), h.close();
              } };
            })(v);
          };
        };
      }, 35: function(c, r, s) {
        Object.defineProperty(r, "__esModule", { value: !0 }), r.unary = void 0;
        var o = s(65), y = s(934);
        r.unary = function(a, f) {
          if (a.responseStream) throw new Error(".unary cannot be used with server-streaming methods. Use .invoke or .client instead.");
          if (a.requestStream) throw new Error(".unary cannot be used with client-streaming methods. Use .client instead.");
          var v = null, u = null, h = y.client(a, { host: f.host, transport: f.transport, debug: f.debug });
          return h.onHeaders((function(l) {
            v = l;
          })), h.onMessage((function(l) {
            u = l;
          })), h.onEnd((function(l, p, i) {
            f.onEnd({ status: l, statusMessage: p, headers: v || new o.Metadata(), message: u, trailers: i });
          })), h.start(f.metadata), h.send(f.request), h.finishSend(), { close: function() {
            h.close();
          } };
        };
      }, 882: function(c, r) {
        Object.defineProperty(r, "__esModule", { value: !0 }), r.frameRequest = void 0, r.frameRequest = function(s) {
          var o = s.serializeBinary(), y = new ArrayBuffer(o.byteLength + 5);
          return new DataView(y, 1, 4).setUint32(0, o.length, !1), new Uint8Array(y, 5).set(o), new Uint8Array(y);
        };
      } }, b = {}, (function c(r) {
        if (b[r]) return b[r].exports;
        var s = b[r] = { exports: {} };
        return n[r].call(s.exports, s, s.exports, c), s.exports;
      })(607);
      var n, b;
    }));
  })(A)), A.exports;
}
var re = Oe(), I = { exports: {} }, Te = I.exports, ne;
function _e() {
  return ne || (ne = 1, (function(e, t) {
    (function(b, c) {
      e.exports = c();
    })(Te, function() {
      return (
        /******/
        (function(n) {
          var b = {};
          function c(r) {
            if (b[r])
              return b[r].exports;
            var s = b[r] = {
              /******/
              i: r,
              /******/
              l: !1,
              /******/
              exports: {}
              /******/
            };
            return n[r].call(s.exports, s, s.exports, c), s.l = !0, s.exports;
          }
          return c.m = n, c.c = b, c.i = function(r) {
            return r;
          }, c.d = function(r, s, o) {
            c.o(r, s) || Object.defineProperty(r, s, {
              /******/
              configurable: !1,
              /******/
              enumerable: !0,
              /******/
              get: o
              /******/
            });
          }, c.n = function(r) {
            var s = r && r.__esModule ? (
              /******/
              function() {
                return r.default;
              }
            ) : (
              /******/
              function() {
                return r;
              }
            );
            return c.d(s, "a", s), s;
          }, c.o = function(r, s) {
            return Object.prototype.hasOwnProperty.call(r, s);
          }, c.p = "", c(c.s = 1);
        })([
          /* 0 */
          /***/
          (function(n, b, c) {
            Object.defineProperty(b, "__esModule", { value: !0 });
            var r = c(3);
            function s(y) {
              return typeof y == "object" && typeof y.headersMap == "object" && typeof y.forEach == "function";
            }
            var o = (function() {
              function y(a, f) {
                a === void 0 && (a = {}), f === void 0 && (f = { splitValues: !1 });
                var v = this;
                if (this.headersMap = {}, a)
                  if (typeof Headers < "u" && a instanceof Headers) {
                    var u = r.getHeaderKeys(a);
                    u.forEach(function(l) {
                      var p = r.getHeaderValues(a, l);
                      p.forEach(function(i) {
                        f.splitValues ? v.append(l, r.splitHeaderValue(i)) : v.append(l, i);
                      });
                    });
                  } else if (s(a))
                    a.forEach(function(l, p) {
                      v.append(l, p);
                    });
                  else if (typeof Map < "u" && a instanceof Map) {
                    var h = a;
                    h.forEach(function(l, p) {
                      v.append(p, l);
                    });
                  } else typeof a == "string" ? this.appendFromString(a) : typeof a == "object" && Object.getOwnPropertyNames(a).forEach(function(l) {
                    var p = a, i = p[l];
                    Array.isArray(i) ? i.forEach(function(d) {
                      v.append(l, d);
                    }) : v.append(l, i);
                  });
              }
              return y.prototype.appendFromString = function(a) {
                for (var f = a.split(`\r
`), v = 0; v < f.length; v++) {
                  var u = f[v], h = u.indexOf(":");
                  if (h > 0) {
                    var l = u.substring(0, h).trim(), p = u.substring(h + 1).trim();
                    this.append(l, p);
                  }
                }
              }, y.prototype.delete = function(a, f) {
                var v = r.normalizeName(a);
                if (f === void 0)
                  delete this.headersMap[v];
                else {
                  var u = this.headersMap[v];
                  if (u) {
                    var h = u.indexOf(f);
                    h >= 0 && u.splice(h, 1), u.length === 0 && delete this.headersMap[v];
                  }
                }
              }, y.prototype.append = function(a, f) {
                var v = this, u = r.normalizeName(a);
                Array.isArray(this.headersMap[u]) || (this.headersMap[u] = []), Array.isArray(f) ? f.forEach(function(h) {
                  v.headersMap[u].push(r.normalizeValue(h));
                }) : this.headersMap[u].push(r.normalizeValue(f));
              }, y.prototype.set = function(a, f) {
                var v = r.normalizeName(a);
                if (Array.isArray(f)) {
                  var u = [];
                  f.forEach(function(h) {
                    u.push(r.normalizeValue(h));
                  }), this.headersMap[v] = u;
                } else
                  this.headersMap[v] = [r.normalizeValue(f)];
              }, y.prototype.has = function(a, f) {
                var v = this.headersMap[r.normalizeName(a)], u = Array.isArray(v);
                if (!u)
                  return !1;
                if (f !== void 0) {
                  var h = r.normalizeValue(f);
                  return v.indexOf(h) >= 0;
                } else
                  return !0;
              }, y.prototype.get = function(a) {
                var f = this.headersMap[r.normalizeName(a)];
                return f !== void 0 ? f.concat() : [];
              }, y.prototype.forEach = function(a) {
                var f = this;
                Object.getOwnPropertyNames(this.headersMap).forEach(function(v) {
                  a(v, f.headersMap[v]);
                }, this);
              }, y.prototype.toHeaders = function() {
                if (typeof Headers < "u") {
                  var a = new Headers();
                  return this.forEach(function(f, v) {
                    v.forEach(function(u) {
                      a.append(f, u);
                    });
                  }), a;
                } else
                  throw new Error("Headers class is not defined");
              }, y;
            })();
            b.BrowserHeaders = o;
          }),
          /* 1 */
          /***/
          (function(n, b, c) {
            Object.defineProperty(b, "__esModule", { value: !0 });
            var r = c(0);
            b.BrowserHeaders = r.BrowserHeaders;
          }),
          /* 2 */
          /***/
          (function(n, b, c) {
            Object.defineProperty(b, "__esModule", { value: !0 });
            function r(o, y) {
              for (var a = o[Symbol.iterator](), f = a.next(); !f.done; )
                y(f.value[0]), f = a.next();
            }
            b.iterateHeaders = r;
            function s(o, y) {
              for (var a = o.keys(), f = a.next(); !f.done; )
                y(f.value), f = a.next();
            }
            b.iterateHeadersKeys = s;
          }),
          /* 3 */
          /***/
          (function(n, b, c) {
            Object.defineProperty(b, "__esModule", { value: !0 });
            var r = c(2);
            function s(u) {
              if (typeof u != "string" && (u = String(u)), /[^a-z0-9\-#$%&'*+.\^_`|~]/i.test(u))
                throw new TypeError("Invalid character in header field name");
              return u.toLowerCase();
            }
            b.normalizeName = s;
            function o(u) {
              return typeof u != "string" && (u = String(u)), u;
            }
            b.normalizeValue = o;
            function y(u, h) {
              var l = u;
              if (l instanceof Headers && l.getAll)
                return l.getAll(h);
              var p = l.get(h);
              return p && typeof p == "string" ? [p] : p;
            }
            b.getHeaderValues = y;
            function a(u) {
              return u;
            }
            function f(u) {
              var h = u, l = {}, p = [];
              return h.keys ? r.iterateHeadersKeys(h, function(i) {
                l[i] || (l[i] = !0, p.push(i));
              }) : h.forEach ? h.forEach(function(i, d) {
                l[d] || (l[d] = !0, p.push(d));
              }) : r.iterateHeaders(h, function(i) {
                var d = i[0];
                l[d] || (l[d] = !0, p.push(d));
              }), p;
            }
            b.getHeaderKeys = f;
            function v(u) {
              var h = [], l = u.split(", ");
              return l.forEach(function(p) {
                p.split(",").forEach(function(i) {
                  h.push(i);
                });
              }), h;
            }
            b.splitHeaderValue = v;
          })
          /******/
        ])
      );
    });
  })(I)), I.exports;
}
var Ce = _e();
class Ne {
  rpc;
  constructor(t) {
    this.rpc = t, this.EchoSwerveSample = this.EchoSwerveSample.bind(this);
  }
  EchoSwerveSample(t, n) {
    return this.rpc.unary(de, P.fromPartial(t), n);
  }
}
const fe = { serviceName: "service.ChoreoService" }, de = {
  methodName: "EchoSwerveSample",
  service: fe,
  requestStream: !1,
  responseStream: !1,
  requestType: {
    serializeBinary() {
      return P.encode(this).finish();
    }
  },
  responseType: {
    deserializeBinary(e) {
      const t = V.decode(e);
      return {
        ...t,
        toObject() {
          return t;
        }
      };
    }
  }
};
class Ae {
  host;
  options;
  constructor(t, n) {
    this.host = t, this.options = n;
  }
  unary(t, n, b) {
    const c = { ...n, ...t.requestType }, r = b && this.options.metadata ? new Ce.BrowserHeaders({ ...this.options?.metadata.headersMap, ...b?.headersMap }) : b ?? this.options.metadata;
    return new Promise((s, o) => {
      re.grpc.unary(t, {
        request: c,
        host: this.host,
        metadata: r ?? {},
        ...this.options.transport !== void 0 ? { transport: this.options.transport } : {},
        debug: this.options.debug ?? !1,
        onEnd: function(y) {
          if (y.status === re.grpc.Code.OK)
            s(y.message.toObject());
          else {
            const a = new ce(y.statusMessage, y.status, y.trailers);
            o(a);
          }
        }
      });
    });
  }
}
class ce extends globalThis.Error {
  constructor(t, n, b) {
    super(t), this.code = n, this.metadata = b;
  }
}
const Pe = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  ChoreoServiceClientImpl: Ne,
  ChoreoServiceDesc: fe,
  ChoreoServiceEchoSwerveSampleDesc: de,
  GrpcWebError: ce,
  GrpcWebImpl: Ae,
  commands: Me
}, Symbol.toStringTag, { value: "Module" }));
export {
  Ie as entity,
  Pe as service
};
//# sourceMappingURL=index.mjs.map
