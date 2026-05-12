interface CodePillProps {
  code: string;
}

export function CodePill({ code }: CodePillProps) {
  return (
    <span className="inline-flex items-center rounded-full border border-sky-200 bg-sky-50 px-2.5 py-1 font-mono text-xs font-semibold tracking-wider text-sky-700">
      {code}
    </span>
  );
}
