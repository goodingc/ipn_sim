type UnwrapArrayTuple<Tuple extends [...any[]]> = {
  [Index in keyof Tuple]: Tuple[Index] extends (infer T)[] ? T : never;
} & { length: Tuple["length"] };

export function zip<Arrays extends [...any[]]>(
  ...arrays: Arrays
): UnwrapArrayTuple<Arrays>[] {
  return [...arrays[0].keys()].map((i) =>
    arrays.map((array) => array[i])
  ) as UnwrapArrayTuple<Arrays>[];
}
