// Smooth scrolling for navigation links
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        document.querySelector(this.getAttribute('href')).scrollIntoView({
            behavior: 'smooth'
        });
    });
});

// Navbar background change on scroll
window.addEventListener('scroll', function() {
    const navbar = document.querySelector('.navbar');
    if (window.scrollY > 50) {
        navbar.classList.add('bg-dark');
        navbar.classList.remove('bg-transparent');
    } else {
        navbar.classList.remove('bg-dark');
        navbar.classList.add('bg-transparent');
    }
});

// Code syntax highlighting (simple implementation)
document.addEventListener('DOMContentLoaded', function() {
    const codeBlocks = document.querySelectorAll('pre code');
    codeBlocks.forEach(block => {
        // Simple syntax highlighting for bash commands
        if (block.textContent.includes('$ ')) {
            block.innerHTML = block.textContent.replace(/\$ (.*?)(\n|$)/g, '<span class="text-primary">$ $1</span>$2');
        }
        // Simple syntax highlighting for YAML
        if (block.textContent.includes(': ')) {
            block.innerHTML = block.textContent.replace(/(\w+):/g, '<span class="text-info">$1:</span>');
        }
    });
});

// Animated counter for stats
function animateCounter(element, target, duration) {
    let start = 0;
    const increment = target / (duration / 16);
    
    function updateCounter() {
        start += increment;
        if (start < target) {
            element.textContent = Math.floor(start);
            requestAnimationFrame(updateCounter);
        } else {
            element.textContent = target;
        }
    }
    
    updateCounter();
}

// Initialize counters when they come into view
const observerOptions = {
    threshold: 0.1
};

const observer = new IntersectionObserver(function(entries) {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const counter = entry.target;
            const target = parseInt(counter.getAttribute('data-target'));
            const duration = 2000;
            animateCounter(counter, target, duration);
            observer.unobserve(counter);
        }
    });
}, observerOptions);

// Observe all counter elements
document.querySelectorAll('.counter').forEach(counter => {
    observer.observe(counter);
});

// Mobile menu toggle
const mobileMenuButton = document.querySelector('.navbar-toggler');
const mobileMenu = document.querySelector('.navbar-collapse');

mobileMenuButton.addEventListener('click', function() {
    mobileMenu.classList.toggle('show');
});

// Close mobile menu when a link is clicked
document.querySelectorAll('.navbar-nav .nav-link').forEach(link => {
    link.addEventListener('click', function() {
        if (mobileMenu.classList.contains('show')) {
            mobileMenu.classList.remove('show');
        }
    });
});

// Copy code functionality
function addCopyButtons() {
    const codeBlocks = document.querySelectorAll('pre');
    codeBlocks.forEach(block => {
        const button = document.createElement('button');
        button.className = 'btn btn-sm btn-outline-secondary copy-button';
        button.textContent = 'Copy';
        button.style.position = 'absolute';
        button.style.top = '10px';
        button.style.right = '10px';
        button.style.zIndex = '10';
        
        block.style.position = 'relative';
        block.appendChild(button);
        
        button.addEventListener('click', function() {
            const code = block.querySelector('code').textContent;
            navigator.clipboard.writeText(code).then(function() {
                button.textContent = 'Copied!';
                setTimeout(function() {
                    button.textContent = 'Copy';
                }, 2000);
            });
        });
    });
}

// Add copy buttons when page loads
document.addEventListener('DOMContentLoaded', addCopyButtons);

// Image modal functionality
function setupImageModals() {
    const images = document.querySelectorAll('img[data-toggle="modal"]');
    images.forEach(img => {
        img.addEventListener('click', function() {
            const modal = document.getElementById('imageModal');
            const modalImg = document.getElementById('modalImage');
            const captionText = document.getElementById('modalCaption');
            
            modal.style.display = 'block';
            modalImg.src = this.src;
            captionText.textContent = this.alt;
        });
    });
    
    // Close modal when clicking on close button or outside
    const closeBtn = document.querySelector('.close');
    if (closeBtn) {
        closeBtn.addEventListener('click', function() {
            document.getElementById('imageModal').style.display = 'none';
        });
    }
    
    window.addEventListener('click', function(event) {
        const modal = document.getElementById('imageModal');
        if (event.target == modal) {
            modal.style.display = 'none';
        }
    });
}

// Setup image modals when page loads
document.addEventListener('DOMContentLoaded', setupImageModals);