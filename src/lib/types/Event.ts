export type CEvent<E, T> = E & { currentTarget: EventTarget & T };
export type NEvent<T> = Event & { currentTarget: EventTarget & T };
