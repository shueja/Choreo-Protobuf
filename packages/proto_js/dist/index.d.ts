import { BinaryReader } from '@bufbuild/protobuf/wire';
import { BinaryWriter } from '@bufbuild/protobuf/wire';
import { grpc } from '@improbable-eng/grpc-web';

declare type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_10 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_11 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_12 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_13 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_14 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_15 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_16 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_17 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_18 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_2 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_3 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_4 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_5 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_6 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_7 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_8 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare type Builtin_9 = Date | Function | Uint8Array | string | number | boolean | undefined;

declare interface ChoreoService {
    EchoSwerveSample(request: DeepPartial_17<EchoSwerveSampleRequest>, metadata?: grpc.Metadata): Promise<EchoSwerveSampleResponse>;
    Generate(request: DeepPartial_17<GenerateRequest>, metadata?: grpc.Metadata): Promise<GenerateResponse>;
    GetDefaultTrajectory(request: DeepPartial_17<Empty>, metadata?: grpc.Metadata): Promise<GetDefaultTrajectoryResponse>;
}

declare class ChoreoServiceClientImpl implements ChoreoService {
    private readonly rpc;
    constructor(rpc: Rpc);
    EchoSwerveSample(request: DeepPartial_17<EchoSwerveSampleRequest>, metadata?: grpc.Metadata): Promise<EchoSwerveSampleResponse>;
    Generate(request: DeepPartial_17<GenerateRequest>, metadata?: grpc.Metadata): Promise<GenerateResponse>;
    GetDefaultTrajectory(request: DeepPartial_17<Empty>, metadata?: grpc.Metadata): Promise<GetDefaultTrajectoryResponse>;
}

declare const ChoreoServiceDesc: {
    serviceName: string;
};

declare const ChoreoServiceEchoSwerveSampleDesc: UnaryMethodDefinitionish;

declare const ChoreoServiceGenerateDesc: UnaryMethodDefinitionish;

declare const ChoreoServiceGetDefaultTrajectoryDesc: UnaryMethodDefinitionish;

export declare namespace commands {
    export {
        EchoSwerveSampleRequest,
        EchoSwerveSampleResponse,
        GenerateRequest,
        GenerateResponse,
        GetDefaultTrajectoryResponse
    }
}

export declare namespace constraint {
    export {
        maxvelocity,
        max_acceleration,
        ExprConstraint
    }
}

declare type DeepPartial<T> = T extends Builtin ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;

declare type DeepPartial_10<T> = T extends Builtin_10 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_10<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_10<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_10<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_10<T[K]>;
} : Partial<T>;

declare type DeepPartial_11<T> = T extends Builtin_11 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_11<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_11<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_11<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_11<T[K]>;
} : Partial<T>;

declare type DeepPartial_12<T> = T extends Builtin_12 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_12<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_12<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_12<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_12<T[K]>;
} : Partial<T>;

declare type DeepPartial_13<T> = T extends Builtin_13 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_13<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_13<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_13<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_13<T[K]>;
} : Partial<T>;

declare type DeepPartial_14<T> = T extends Builtin_14 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_14<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_14<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_14<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_14<T[K]>;
} : Partial<T>;

declare type DeepPartial_15<T> = T extends Builtin_15 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_15<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_15<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_15<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_15<T[K]>;
} : Partial<T>;

declare type DeepPartial_16<T> = T extends Builtin_16 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_16<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_16<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_16<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_16<T[K]>;
} : Partial<T>;

declare type DeepPartial_17<T> = T extends Builtin_17 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_17<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_17<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_17<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_17<T[K]>;
} : Partial<T>;

declare type DeepPartial_18<T> = T extends Builtin_18 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_18<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_18<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_18<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_18<T[K]>;
} : Partial<T>;

declare type DeepPartial_2<T> = T extends Builtin_2 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_2<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_2<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_2<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_2<T[K]>;
} : Partial<T>;

declare type DeepPartial_3<T> = T extends Builtin_3 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_3<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_3<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_3<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_3<T[K]>;
} : Partial<T>;

declare type DeepPartial_4<T> = T extends Builtin_4 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_4<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_4<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_4<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_4<T[K]>;
} : Partial<T>;

declare type DeepPartial_5<T> = T extends Builtin_5 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_5<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_5<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_5<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_5<T[K]>;
} : Partial<T>;

declare type DeepPartial_6<T> = T extends Builtin_6 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_6<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_6<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_6<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_6<T[K]>;
} : Partial<T>;

declare type DeepPartial_7<T> = T extends Builtin_7 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_7<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_7<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_7<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_7<T[K]>;
} : Partial<T>;

declare type DeepPartial_8<T> = T extends Builtin_8 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_8<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_8<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_8<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_8<T[K]>;
} : Partial<T>;

declare type DeepPartial_9<T> = T extends Builtin_9 ? T : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial_9<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial_9<U>> : T extends {
    $case: string;
    value: unknown;
} ? {
    $case: T["$case"];
    value?: DeepPartial_9<T["value"]>;
} : T extends {} ? {
    [K in keyof T]?: DeepPartial_9<T[K]>;
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

declare const DifferentialSample: MessageFns_9<DifferentialSample>;

declare interface DifferentialTrajectory {
    samples: DifferentialSample[];
}

declare const DifferentialTrajectory: MessageFns_12<DifferentialTrajectory>;

declare enum DriveType {
    DRIVE_TYPE_SWERVE = 0,
    DRIVE_TYPE_DIFFERENTIAL = 1,
    DRIVE_TYPE_MECANUM = 2,
    UNRECOGNIZED = -1
}

declare function driveTypeFromJSON(object: any): DriveType;

declare function driveTypeToJSON(object: DriveType): string;

declare interface EchoSwerveSampleRequest {
    sample: SwerveSample | null;
}

declare const EchoSwerveSampleRequest: MessageFns_14<EchoSwerveSampleRequest>;

declare interface EchoSwerveSampleResponse {
    sample: SwerveSample | null;
}

declare const EchoSwerveSampleResponse: MessageFns_14<EchoSwerveSampleResponse>;

/**
 * A generic empty message that you can re-use to avoid defining duplicated
 * empty messages in your APIs. A typical example is to use it as the request
 * or the response type of an API method. For instance:
 *
 *     service Foo {
 *       rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
 *     }
 */
declare interface Empty {
}

declare const Empty: MessageFns_17<Empty>;

export declare namespace entity {
    export {
        parameters,
        DifferentialSample,
        driveTypeFromJSON,
        driveTypeToJSON,
        DriveType,
        Expr,
        ProjectFile,
        ForceVector,
        SwerveSample,
        SwerveTrajectory,
        DifferentialTrajectory,
        GenerationOutput,
        TrajectoryFile
    }
}

declare type Exact<P, I extends P> = P extends Builtin ? P : P & {
    [K in keyof P]: Exact<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion<P>>]: never;
};

declare type Exact_10<P, I extends P> = P extends Builtin_10 ? P : P & {
    [K in keyof P]: Exact_10<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_10<P>>]: never;
};

declare type Exact_11<P, I extends P> = P extends Builtin_11 ? P : P & {
    [K in keyof P]: Exact_11<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_11<P>>]: never;
};

declare type Exact_12<P, I extends P> = P extends Builtin_12 ? P : P & {
    [K in keyof P]: Exact_12<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_12<P>>]: never;
};

declare type Exact_13<P, I extends P> = P extends Builtin_13 ? P : P & {
    [K in keyof P]: Exact_13<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_13<P>>]: never;
};

declare type Exact_14<P, I extends P> = P extends Builtin_14 ? P : P & {
    [K in keyof P]: Exact_14<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_14<P>>]: never;
};

declare type Exact_15<P, I extends P> = P extends Builtin_15 ? P : P & {
    [K in keyof P]: Exact_15<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_15<P>>]: never;
};

declare type Exact_16<P, I extends P> = P extends Builtin_16 ? P : P & {
    [K in keyof P]: Exact_16<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_16<P>>]: never;
};

declare type Exact_17<P, I extends P> = P extends Builtin_18 ? P : P & {
    [K in keyof P]: Exact_17<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_17<P>>]: never;
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

declare type Exact_4<P, I extends P> = P extends Builtin_4 ? P : P & {
    [K in keyof P]: Exact_4<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_4<P>>]: never;
};

declare type Exact_5<P, I extends P> = P extends Builtin_5 ? P : P & {
    [K in keyof P]: Exact_5<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_5<P>>]: never;
};

declare type Exact_6<P, I extends P> = P extends Builtin_6 ? P : P & {
    [K in keyof P]: Exact_6<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_6<P>>]: never;
};

declare type Exact_7<P, I extends P> = P extends Builtin_7 ? P : P & {
    [K in keyof P]: Exact_7<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_7<P>>]: never;
};

declare type Exact_8<P, I extends P> = P extends Builtin_8 ? P : P & {
    [K in keyof P]: Exact_8<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_8<P>>]: never;
};

declare type Exact_9<P, I extends P> = P extends Builtin_9 ? P : P & {
    [K in keyof P]: Exact_9<P[K], I[K]>;
} & {
    [K in Exclude<keyof I, KeysOfUnion_9<P>>]: never;
};

declare interface Expr {
    value: number;
    expr: string;
}

declare const Expr: MessageFns<Expr>;

declare interface ExprBumper {
    front: Expr | null;
    left: Expr | null;
    right: Expr | null;
    back: Expr | null;
}

declare const ExprBumper: MessageFns_7<ExprBumper>;

declare interface ExprConstraint {
    enabled: boolean;
    from: WaypointID | null;
    to: WaypointID | null;
    /**
     * ExprMaxVelocity maxvelocity = 4;
     * ExprMaxAcceleration max_acceleration = 5;
     */
    data?: {
        $case: "maxVelocity";
        value: ExprMaxVelocity;
    } | {
        $case: "maxAcceleration";
        value: ExprMaxAcceleration;
    } | null;
}

declare const ExprConstraint: MessageFns_5<ExprConstraint>;

declare interface ExprMaxAcceleration {
    max: Expr | null;
}

declare const ExprMaxAcceleration: MessageFns_3<ExprMaxAcceleration>;

declare interface ExprMaxVelocity {
    max: Expr | null;
}

declare const ExprMaxVelocity: MessageFns_2<ExprMaxVelocity>;

declare interface ExprModule {
    x: Expr | null;
    y: Expr | null;
}

declare const ExprModule: MessageFns_7<ExprModule>;

declare interface ExprParameters {
    targetDt: Expr | null;
    waypoints: ExprWaypoint[];
    constraints: ExprConstraint[];
}

declare const ExprParameters: MessageFns_8<ExprParameters>;

declare interface ExprRobotConfig {
    mass: Expr | null;
    inertia: Expr | null;
    gearing: Expr | null;
    radius: Expr | null;
    vmax: Expr | null;
    tmax: Expr | null;
    cof: Expr | null;
    differentialTrackWidth: Expr | null;
    /**
     * Expr bumper_front = 9;
     * Expr bumper_left = 10;
     * Expr bumper_back = 11;
     * Expr bumper_right = 12;
     * Expr fl_x = 13;
     * Expr fl_y = 14;
     * Expr fr_x = 15;
     * Expr fl_
     */
    bumper: ExprBumper | null;
    frontLeft: ExprModule | null;
    frontRight: ExprModule | null;
    backLeft: ExprModule | null;
    backRight: ExprModule | null;
}

declare const ExprRobotConfig: MessageFns_7<ExprRobotConfig>;

declare interface ExprWaypoint {
    x: Expr | null;
    y: Expr | null;
    heading: Expr | null;
    intervals: number;
    split: boolean;
    fixTranslation: boolean;
    fixHeading: boolean;
    overrideIntervals: boolean;
}

declare const ExprWaypoint: MessageFns_6<ExprWaypoint>;

declare interface ForceVector {
    x: number;
    y: number;
}

declare const ForceVector: MessageFns_11<ForceVector>;

declare interface GenerateRequest {
    trajectory: TrajectoryFile | null;
}

declare const GenerateRequest: MessageFns_15<GenerateRequest>;

declare interface GenerateResponse {
    trajectory: TrajectoryFile | null;
}

declare const GenerateResponse: MessageFns_15<GenerateResponse>;

declare interface GenerationOutput {
    trajectory?: {
        $case: "swerve";
        value: SwerveTrajectory;
    } | {
        $case: "differential";
        value: DifferentialTrajectory;
    } | null;
    splits: number[];
    waypoints: number[];
    config?: ExprRobotConfig | null | undefined;
}

declare const GenerationOutput: MessageFns_12<GenerationOutput>;

declare interface GetDefaultTrajectoryResponse {
    trajectory: TrajectoryFile | null;
}

declare const GetDefaultTrajectoryResponse: MessageFns_16<GetDefaultTrajectoryResponse>;

export declare namespace google {
    export {
        protobuf
    }
}

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

declare type KeysOfUnion_10<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_11<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_12<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_13<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_14<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_15<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_16<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_17<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_2<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_3<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_4<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_5<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_6<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_7<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_8<T> = T extends T ? keyof T : never;

declare type KeysOfUnion_9<T> = T extends T ? keyof T : never;

export declare namespace max_acceleration {
    export {
        ExprMaxAcceleration
    }
}

export declare namespace maxvelocity {
    export {
        ExprMaxVelocity,
        TestExpr
    }
}

declare interface MessageFns<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact<DeepPartial<T>, I>>(base?: I): T;
    fromPartial<I extends Exact<DeepPartial<T>, I>>(object: I): T;
}

declare interface MessageFns_10<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_10<DeepPartial_10<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_10<DeepPartial_10<T>, I>>(object: I): T;
}

declare interface MessageFns_11<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_11<DeepPartial_11<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_11<DeepPartial_11<T>, I>>(object: I): T;
}

declare interface MessageFns_12<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_12<DeepPartial_12<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_12<DeepPartial_12<T>, I>>(object: I): T;
}

declare interface MessageFns_13<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_13<DeepPartial_13<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_13<DeepPartial_13<T>, I>>(object: I): T;
}

declare interface MessageFns_14<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_14<DeepPartial_14<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_14<DeepPartial_14<T>, I>>(object: I): T;
}

declare interface MessageFns_15<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_15<DeepPartial_15<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_15<DeepPartial_15<T>, I>>(object: I): T;
}

declare interface MessageFns_16<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_16<DeepPartial_16<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_16<DeepPartial_16<T>, I>>(object: I): T;
}

declare interface MessageFns_17<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_17<DeepPartial_18<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_17<DeepPartial_18<T>, I>>(object: I): T;
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

declare interface MessageFns_4<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_4<DeepPartial_4<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_4<DeepPartial_4<T>, I>>(object: I): T;
}

declare interface MessageFns_5<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_5<DeepPartial_5<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_5<DeepPartial_5<T>, I>>(object: I): T;
}

declare interface MessageFns_6<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_6<DeepPartial_6<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_6<DeepPartial_6<T>, I>>(object: I): T;
}

declare interface MessageFns_7<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_7<DeepPartial_7<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_7<DeepPartial_7<T>, I>>(object: I): T;
}

declare interface MessageFns_8<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_8<DeepPartial_8<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_8<DeepPartial_8<T>, I>>(object: I): T;
}

declare interface MessageFns_9<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;
    decode(input: BinaryReader | Uint8Array, length?: number): T;
    fromJSON(object: any): T;
    toJSON(message: T): unknown;
    create<I extends Exact_9<DeepPartial_9<T>, I>>(base?: I): T;
    fromPartial<I extends Exact_9<DeepPartial_9<T>, I>>(object: I): T;
}

export declare namespace parameters {
    export {
        constraint,
        waypoint,
        robotconfig,
        WaypointIDFirst,
        WaypointIDLast,
        WaypointIDX,
        WaypointID,
        ExprParameters
    }
}

declare interface ProjectFile {
    name: string;
    config: ExprRobotConfig | null;
    driveType: DriveType;
}

declare const ProjectFile: MessageFns_10<ProjectFile>;

export declare namespace protobuf {
    export {
        Empty
    }
}

export declare namespace robotconfig {
    export {
        ExprModule,
        ExprBumper,
        ExprRobotConfig
    }
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
        ChoreoServiceGenerateDesc,
        ChoreoServiceGetDefaultTrajectoryDesc,
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
    fl: ForceVector | null;
    fr: ForceVector | null;
    bl: ForceVector | null;
    br: ForceVector | null;
}

declare const SwerveSample: MessageFns_11<SwerveSample>;

declare interface SwerveTrajectory {
    samples: SwerveSample[];
}

declare const SwerveTrajectory: MessageFns_12<SwerveTrajectory>;

declare interface TestExpr {
    test: string;
}

declare const TestExpr: MessageFns_2<TestExpr>;

declare interface TrajectoryFile {
    name: string;
    params: ExprParameters | null;
    snapshot?: ExprParameters | null | undefined;
    trajectory?: GenerationOutput | null | undefined;
}

declare const TrajectoryFile: MessageFns_13<TrajectoryFile>;

declare type UnaryMethodDefinitionish = UnaryMethodDefinitionishR;

declare interface UnaryMethodDefinitionishR extends grpc.UnaryMethodDefinition<any, any> {
    requestStream: any;
    responseStream: any;
}

export declare namespace waypoint {
    export {
        ExprWaypoint
    }
}

declare interface WaypointID {
    id?: {
        $case: "first";
        value: WaypointIDFirst;
    } | {
        $case: "last";
        value: WaypointIDLast;
    } | {
        $case: "idx";
        value: WaypointIDX;
    } | null;
}

declare const WaypointID: MessageFns_4<WaypointID>;

declare interface WaypointIDFirst {
}

declare const WaypointIDFirst: MessageFns_4<WaypointIDFirst>;

declare interface WaypointIDLast {
}

declare const WaypointIDLast: MessageFns_4<WaypointIDLast>;

declare interface WaypointIDX {
    idx: number;
}

declare const WaypointIDX: MessageFns_4<WaypointIDX>;

export { }
