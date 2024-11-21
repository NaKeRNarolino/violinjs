export function encode(str: string): Uint8Array {
  return new TextEncoder().encode(`${str}\0`);
}
