document.addEventListener('DOMContentLoaded', () => {
    // Toggle hamburger menu on click
    for (const el of document.querySelectorAll('.navbar-burger')) {
        el.addEventListener('click', () => {
            const target = document.getElementById(el.dataset.target);

            el.classList.toggle('is-active');
            target.classList.toggle('is-active');
        });
    }

    // Hide notifications on click
    for (const button of document.querySelectorAll('.notification.toggleable .delete')) {
        const notification = button.parentNode;

        button.addEventListener('click', () => {
            notification.parentNode.removeChild(notification);
        });
    }
});
