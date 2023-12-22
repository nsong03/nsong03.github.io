// Get all section headers
var sectionHeaders = document.querySelectorAll('.toggle-header');

// Attach click event listener to each section header
sectionHeaders.forEach(function(header) {
  header.addEventListener('click', function() {
    var content = this.nextElementSibling;
    content.classList.toggle('show');
    var isVisible = content.classList.contains('show');
    this.querySelector('.toggle-icon').textContent = isVisible ? '▲' : '▼';
  });

  // Set initial toggle button text based on display state
  var content = header.nextElementSibling;
  var isVisible = getComputedStyle(content).display !== 'none';
  var toggleIcon = header.querySelector('.toggle-icon');
  toggleIcon.textContent = isVisible ? '▲' : '▼';
});
