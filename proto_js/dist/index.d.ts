import { BinaryReader } from '@bufbuild/protobuf/wire';
import { BinaryWriter } from '@bufbuild/protobuf/wire';
import { grpc } from '@improbable-eng/grpc-web';

declare type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_2 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_3 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_4 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare interface ChoreoService {
    EchoSwerveSample(request: DeepPartial_4<EchoSwerveSampleRequest>, metadata?: grpc.Metadata): Promise<EchoSwerveSampleResponse>;
}

declare class ChoreoServiceClientImpl implements ChoreoService {
    private readonly rpc;
    constructor(rpc: Rpc);
    EchoSwerveSample(request: DeepPartial_4<EchoSwerveSampleRequest>, metadata?: grpc.Metadata): Promise<EchoSwerveSampleResponse>;
}

declare const ChoreoServiceDesc: {
    serviceName: string;
};

declare const ChoreoServiceEchoSwerveSampleDesc: UnaryMethodDefinitionish;

export declare namespace commands {
    export {
        EchoSwerveSampleRequest,
        EchoSwerveSampleResponse
    }
}

declare type DeepPartial<T> = T extends Builtin ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;

declare type DeepPartial_2<T> = T extends Builtin_2 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_2<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_2<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial_2<T[K]>;
} : Partial<T>;

declare type DeepPartial_3<T> = T extends Builtin_3 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_3<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_3<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial_3<T[K]>;
} : Partial<T>;

declare type DeepPartial_4<T> = T extends Builtin_4 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_4<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_4<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial_4<T[K]>;
} : Partial<T>;

declare interface DifferentialSample {
    t: number;
    x: number;
    y: number;
    heading: number;
    vl: number;
    vr: number;
    omega: number;
    al: number;
    ar: number;
    alpha: number;
    fl: number;
    fr: number;
}

declare const DifferentialSample: MessageFns<DifferentialSample>;

declare interface EchoSwerveSampleRequest {
    sample: SwerveSample | undefined;
}

declare const EchoSwerveSampleRequest: MessageFns_3<EchoSwerveSampleRequest>;

declare interface EchoSwerveSampleResponse {
    sample: SwerveSample | undefined;
}

declare const EchoSwerveSampleResponse: MessageFns_3<EchoSwerveSampleResponse>;

export declare namespace entity {
    export {
        DifferentialSample,
        ForceVector,
        SwerveSample
    }
}

declare type Exact<P, I extends P> = P extends Builtin ? P : P & {
    [K in keyof P]: Exact<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion<P>>]: never;
};

declare type Exact_2<P, I extends P> = P extends Builtin_2 ? P : P & {
    [K in keyof P]: Exact_2<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_2<P>>]: never;
};

declare type Exact_3<P, I extends P> = P extends Builtin_3 ? P : P & {
    [K in keyof P]: Exact_3<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_3<P>>]: never;
};

declare interface ForceVector {
    x: number;
    y: number;
}

declare const ForceVector: MessageFns_2<ForceVector>;

declare class GrpcWebError extends globalThis.Error {
    code: grpc.Code;
    metadata: grpc.Metadata;
    constructor(message: string, code: grpc.Code, metadata: grpc.Metadata);
}

declare class GrpcWebImpl {
    private host;
    private options;
    constructor(host: string, options: {
        transport?: grpc.TransportFactory;
        debug?: boolean;
        metadata?: grpc.Metadata;
        upStreamRetryCodes?: number[];
    });
    unary<T extends UnaryMethodDefinitionish>(methodDesc: T, _request: any, metadata: grpc.Metadata | undefined): Promise<any>;
}

declare type KeysOfUnion<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_2<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_3<T> = T extends T ? keyof T : never;

declare interface MessageFns<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact<DeepPartial<T>, I>>(base?: I): T;
    fromPartial<I extends Exact<DeepPartial<T>, I>>(object: I): T;
}

declare interface MessageFns_2<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_2<DeepPartial_2<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_2<DeepPartial_2<T>, I>>(object: I): T;
}

declare interface MessageFns_3<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_3<DeepPartial_3<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_3<DeepPartial_3<T>, I>>(object: I): T;
}

declare interface Rpc {
    unary<T extends UnaryMethodDefinitionish>(methodDesc: T, request: any, metadata: grpc.Metadata | undefined): Promise<any>;
}

export declare namespace service {
    export {
        commands,
        ChoreoService,
        ChoreoServiceClientImpl,
        ChoreoServiceDesc,
        ChoreoServiceEchoSwerveSampleDesc,
        GrpcWebImpl,
        GrpcWebError
    }
}

declare interface SwerveSample {
    t: number;
    x: number;
    y: number;
    heading: number;
    vx: number;
    vy: number;
    omega: number;
    ax: number;
    ay: number;
    alpha: number;
    fl: ForceVector | undefined;
    fr: ForceVector | undefined;
    bl: ForceVector | undefined;
    br: ForceVector | undefined;
}

declare const SwerveSample: MessageFns_2<SwerveSample>;

declare type UnaryMethodDefinitionish = UnaryMethodDefinitionishR;

declare interface UnaryMethodDefinitionishR extends grpc.UnaryMethodDefinition<any, any> {
    requestStream: any;
    responseStream: any;
}

export { }
