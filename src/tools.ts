export function pickProps<T, K extends keyof T>(source: T, ...props: K[]): Pick<T, K> {
  return props.reduce((target, prop) => {
    target[prop] = source[prop];
    return target;
  }, {} as Pick<T, K>)
}