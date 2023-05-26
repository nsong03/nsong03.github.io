// Get all section headers
var sectionHeaders = document.querySelectorAll('.toggle-header');

// Attach click event listener to each section header
sectionHeaders.forEach(function(header) {
  header.addEventListener('click', function() {
    var content = this.nextElementSibling;
    content.style.display = (content.style.display === 'none') ? 'block' : 'none';
    this.querySelector('.toggle-icon').textContent = (content.style.display === 'none') ? '▼' : '▲';
  });
});