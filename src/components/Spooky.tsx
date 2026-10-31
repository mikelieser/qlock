// Decorations for the 'katja' theme: a dangling spider, one crawling across the face, a bat past the moon.

function Spider() {
  return (
    <svg class="spider" viewBox="-20 -20 40 40" aria-hidden="true">
      <g class="legs legs-a">
        <path d="M-3 -2 L-10 -9 L-13 -15" />
        <path d="M-4 3 L-12 2 L-17 6" />
        <path d="M3 0 L11 -3 L16 -1" />
        <path d="M3 6 L10 11 L12 17" />
      </g>
      <g class="legs legs-b">
        <path d="M-3 0 L-11 -3 L-16 -1" />
        <path d="M-3 6 L-10 11 L-12 17" />
        <path d="M3 -2 L10 -9 L13 -15" />
        <path d="M4 3 L12 2 L17 6" />
      </g>
      <ellipse class="body" cx="0" cy="5" rx="5.5" ry="7" />
      <circle class="body" cx="0" cy="-4.5" r="3.6" />
      <circle class="eye" cx="-1.4" cy="-5.4" r="0.9" />
      <circle class="eye" cx="1.4" cy="-5.4" r="0.9" />
    </svg>
  );
}

export function Spooky() {
  return (
    <div class="spooky" aria-hidden="true">
      <div class="spider-drop">
        <Spider />
      </div>
      <div class="spider-walk">
        <Spider />
      </div>
      <div class="bat-path">
        <svg class="bat" viewBox="0 0 100 50">
          <path d="M50 20 C47 12 41 10 36 13 C31 5 19 3 5 9 C12 13 14 19 12 25 C18 21 25 23 29 29 C33 25 39 25 43 31 C45 27 47 26 50 27 C53 26 55 27 57 31 C61 25 67 25 71 29 C75 23 82 21 88 25 C86 19 88 13 95 9 C81 3 69 5 64 13 C59 10 53 12 50 20 Z M46 14 L47 8 L49 13 Z M54 14 L53 8 L51 13 Z" />
        </svg>
      </div>
    </div>
  );
}
