import { FC, useRef, SyntheticEvent, useEffect } from "react";

type EditState = {
  selectionEnd: number;
  length: number;
  text: string;
};

export type SelectEvent = {
  start: number;
  end: number;
};

export type ChangeEvent = {
  start: number;
  end: number;
  text: string;
};

interface TextareaProps {
  value: string;
  selectionStart: number;
  selectionEnd: number;
  onSelect: (selection: SelectEvent) => void;
  onChange: (change: ChangeEvent) => void;
  className: string;
}

// We need to monitor these for performance. If this becomes a problem
// the codepoint offset <-> byte offset conversion can be done in the
// core and exposed in the interface

function ucToBytes(value: string, index: number): number {
  return Array.from(value).slice(0, index).join("").length;
}

function bytesToUc(value: string, index: number): number {
  return Array.from(value.substring(0, index)).length;
}

const Textarea: FC<TextareaProps> = ({
  value,
  selectionStart,
  selectionEnd,
  onSelect,
  onChange,
