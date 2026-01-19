import {service} from "@choreo/proto"
import './App.css'
import { grpc } from '@improbable-eng/grpc-web';
type Valid<T, R = Required<T>> = T extends object ? {
  [K in keyof R]: Valid<NonNullable<R[K]>>
} : T;
function validate<T>(obj: T) : Valid<T> | undefined {
  if (! (obj instanceof Object)) {
    return obj as Valid<T>;
  }
  for (const key in (obj as object)) {
    if (key === undefined) {
      return undefined;
    }
  }
  return obj as Valid<T>;
}
function validateThrow<T>(obj: T) : Valid<T> {
    const valid = validate<T>(obj);
    if (valid === undefined) {
        throw new TypeError();
    }
    return valid;
}
function makeCommands(rpc: ConstructorParameters<typeof service.ChoreoServiceClientImpl>[0]) {
const client: Client  = new service.ChoreoServiceClientImpl(rpc);

return {
    EchoSwerveSample: (request) => client.EchoSwerveSample(request).then(validateThrow),
    Generate: (request) => client.Generate(request).then(validateThrow),
    GetDefaultTrajectory: () => client.GetDefaultTrajectory({}).then(validateThrow)
} as const satisfies {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  [N in keyof Client] : (request: Parameters<Client[N]>[0])=>any;
};
}
const rpc = new service.GrpcWebImpl('http://localhost:50051', {
  debug: false,
  metadata: new grpc.Metadata({ SomeHeader: 'bar' }),
});
type Client = service.ChoreoService;
export const Commands = makeCommands(rpc);




