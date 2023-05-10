export type SMEvent<T> = MouseEvent & { currentTarget: EventTarget & T };
export type SNEvent<T> = Event & { currentTarget: EventTarget & T };
export type SFEvent<T> = FocusEvent & { currentTarget: EventTarget & T };
