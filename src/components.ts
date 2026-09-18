class CloseButton extends HTMLElement {
  connectedCallback() {
    // Extract relevant attributes from the tag
    const id = this.getAttribute('id') || '';
    const extraClasses = this.getAttribute('class') || '';
    const ariaLabel = this.getAttribute('aria-label') || 'Close';

    this.innerHTML = `
      <button class="close-button ${extraClasses}" ${id ? `id="\${id}"` : ''} type="button" aria-label="${ariaLabel}">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://w3.org">
          <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </button>
    `;

    this.style.display = 'contents';
  }
}

customElements.define('ui-close-button', CloseButton);
