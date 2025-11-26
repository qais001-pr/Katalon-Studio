<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body</name>
   <tag></tag>
   <elementGuidId>3e86c8c2-07af-47e0-8f85-9b66294ad099</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>internal:text=&quot;Loading... Welcome to BIIT Admission Portal Remember me Forgot password? SIGN IN&quot;i</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>9c84488f-32e9-48ec-b076-14c92215518a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
      
        Loading...
    

    
        
            
            
                Welcome to 
                BIIT 
                Admission Portal
            
            
                
                
            
            
                
                    
                         Remember me
                    
                
                
                    
                        Forgot password?
                
            
            SIGN IN
            
                Don't have an account? Register Now
            
        
        
            function checkInternetStatus(callback) {
                const xhr = new XMLHttpRequest();
                xhr.open('GET', 'https://jsonplaceholder.typicode.com/posts', true);

                xhr.onload = function() {
                    if (xhr.status >= 200 &amp;&amp; xhr.status &lt; 300) {
                        callback(0); // Online
                    } else {
                        callback(1); // Offline
                    }
                };

                xhr.onerror = function() {
                    callback(1); // Offline
                };

                xhr.send();
            }

            // Example usage:
            checkInternetStatus(function(status) {
                if (status === 0) {
                    console.log(&quot;Online&quot;);
                } else {
                    console.log(&quot;Offline&quot;);
                }
            });
        
      
    function getUrlParameter(name) {
    name = name.replace(/[\[]/, '\\[').replace(/[\]]/, '\\]');
    var regex = new RegExp('[\\?&amp;]' + name + '=([^&amp;#]*)');
    var results = regex.exec(window.location.search);
    return results === null ? null : decodeURIComponent(results[1].replace(/\+/g, ' '));
}

// Fetch role from the server based on email and compare with the role from the URL
function checkRoleAndSubmit(email, role, callback) {
    // Make an AJAX request to the server to fetch the user's role based on the email and role
    fetch('/check-role.php', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ email: email, role: role }) // Pass both email and expected role to the server
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            if (data.match === 1) {
                // Role matched, trigger the callback (submit button click)
                callback();
            } else { 
    preloader.style.display = 'none'; 
    // Role mismatch, show error and don't submit
    Swal.fire({
        icon: 'error',
        title: 'Error',
        text: 'An error occurred. Please contact Administrator.'
    }).then((result) => {
        if (result.isConfirmed) {
            // Go back to the previous page when &quot;OK&quot; is clicked
            window.history.back();
        }
    });
}

        } else {
            Swal.fire({
                icon: 'error',
                title: 'Error',
                text: data.message || 'No user found with this email!'
            });
        }
    })
    .catch(error => {
        Swal.fire({
            icon: 'error',
            title: 'Error',
            text: 'An error occurred while checking the role!'
        });
        console.error('Error:', error);
    });
}

// Helper function to get URL parameters
function getUrlParameter(name) {
    name = name.replace(/[\[\]]/g, '\\$&amp;');
    const regex = new RegExp('[?&amp;]' + name + '(=([^&amp;#]*)|&amp;|#|$)');
    const results = regex.exec(window.location.href);
    if (!results) return null;
    if (!results[2]) return '';
    return decodeURIComponent(results[2].replace(/\+/g, ' '));
}

// Fill email, role, and password from URL if they exist, and trigger auto-submit
window.onload = function() {
    const usernameField = document.getElementById('username');
    const passwordField = document.getElementById('password');
    const submitBtn = document.getElementById('submitBtn');
    const preloader = document.getElementById('preloader');
    const content = document.querySelector('.container');

    // Initially hide the content and show preloader if parameters are present
    content.style.display = 'none';
    preloader.style.display = 'none';  // Ensure preloader is hidden initially

    // Get the parameters from the URL
    const email = getUrlParameter('email');
    const password = getUrlParameter('password');
    const role = getUrlParameter('role');

    if (email &amp;&amp; password &amp;&amp; role) {
        usernameField.value = email;
        passwordField.value = password;

        // Show preloader only if parameters are present
        preloader.style.display = 'flex';

        // Check role from the server and trigger submit if valid
        checkRoleAndSubmit(email, role, function() {
            submitBtn.click();  // Automatically trigger the submit button if role matches
        });
    } else {
        // If no email/password in URL, show content immediately
        content.style.display = 'block';
    }
};



    document.getElementById('submitBtn').addEventListener('click', function () {
        const preloader = document.getElementById('preloader');
        preloader.style.display = 'flex'; // Show preloader during form submission
    });

    const name = document.getElementById('username');
    const password = document.getElementById(&quot;password&quot;);

    function accountLogin(event) {
        event.preventDefault();
        let timerInterval;
        Swal.fire({
            title: 'Logging In!',
            html: 'Please wait...&lt;b>&lt;/b>',
            timerProgressBar: true,
            didOpen: () => {
                Swal.showLoading();
                const b = Swal.getHtmlContainer().querySelector('b');
                timerInterval = setInterval(() => {
                    b.textContent = Swal.getTimerLeft();
                }, 100);
            },
            willClose: () => {
                clearInterval(timerInterval);
            }
        });

        // Call checkInternetStatus with a callback function
        checkInternetStatus(function(status) {
            if (status === 0) {
                $.ajax({
                    url: &quot;/index.php&quot;,
                    type: &quot;POST&quot;,
                    data: {
                        method: 'signin',
                        username: name.value,
                        password: password.value,
                    },
                    success: function(response) {
                        // Hide preloader after response is received
                        Swal.close();  // Close the SweetAlert loading animation
                        const preloader = document.getElementById('preloader');
                        preloader.style.display = 'none';  // Hide preloader

                        console.log(response);
                        if (response == 1) {
                            console.log('PHP Session Id:', &quot;&quot;);
                            Swal.fire({
                                position: 'center',
                                icon: 'success',
                                title: &quot;Login Successful!&quot;,
                                showConfirmButton: false,
                                timer: 3000,
                                allowOutsideClick: false
                            });
                            setTimeout(function() {
                                window.location.href = &quot;login_home.php&quot;;
                            }, 3000);
                        } 
                        
//                          if (response == 1) {
//     console.log('PHP Session Id:', &quot;&quot;);
    
//     Swal.fire({
//         position: 'center',
//         icon: 'info', // Change icon type if needed (e.g., 'warning', 'error')
//         title: &quot;Admission has been closed&quot;,
//         showConfirmButton: false,
//         timer: 3000,
//         allowOutsideClick: false
//     });

//     setTimeout(function() {
//         // window.location.href = &quot;login_home.php&quot;; // Uncomment if you need a redirect
//     }, 3000);
// }
                        else if (response == 2 || response == 3) {
                             console.log(response);
                            console.log('PHP Session Username:', &quot;&quot;);
                            Swal.fire({
                                position: 'center',
                                icon: 'success',
                                title: &quot;Login Successful!&quot;,
                                showConfirmButton: false,
                                timer: 4000,
                                allowOutsideClick: false
                            });
                            setTimeout(function() {
                                window.location.href = &quot;admin_home.php&quot;;
                            }, 2000);
                        } 
                         else if (response == 4 || response == 5) {
                            console.log('PHP Session Username:', &quot;&quot;);
                            console.log(response);
                            Swal.fire({
                                position: 'center',
                                icon: 'success',
                                title: &quot;Login Successful!&quot;,
                                showConfirmButton: false,
                                timer: 4000,
                                allowOutsideClick: false
                            });
                            setTimeout(function() {
                                window.location.href = &quot;Documents.php&quot;;
                            }, 2000);
                        } else if (response == 10) {
                            Swal.fire({
                                position: 'center',
                                icon: 'error',
                                title: &quot;Invalid Username/Password!&quot;,
                                text: &quot;&quot;,
                                showConfirmButton: false,
                                timer: 2000,
                            });
                        }
                    },
                    error: function() {
                        // Hide preloader in case of AJAX error
                        Swal.close();
                        const preloader = document.getElementById('preloader');
                        preloader.style.display = 'none';  // Hide preloader
                        Swal.fire({
                            position: 'center',
                            icon: 'error',
                            title: &quot;Server Error&quot;,
                            text: &quot;Please try again later.&quot;,
                            showConfirmButton: false,
                            timer: 3000,
                        });
                    }
                });

            } else {
                // No internet connection
                Swal.close();
                Swal.fire({
                    position: 'center',
                    icon: 'error',
                    title: &quot;No Internet Connection&quot;,
                    text: &quot;Please check your internet connection and try again.&quot;,
                    showConfirmButton: false,
                    timer: 3000,
                });
            }
        });
    }


        
            function updateOnlineStatus() {
                if (navigator.onLine) {
                    if (confirm('Connection restored! Please reload the page?')) {
                        location.reload();
                    }
                } else {
                    alert('Connection lost!');
                }
            }

            window.addEventListener('online', updateOnlineStatus);
            window.addEventListener('offline', updateOnlineStatus);

            // Initial check
            if (!navigator.onLine) {
                alert('You are currently offline!');
            }
        
    
    
    




/html[1]/body[1]</value>
      <webElementGuid>9b8bae91-4a3b-49e3-967b-b4286efe1fdf</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>65994485-f26c-4382-a382-c32a93855c09</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>55cb4029-0bd5-410c-9052-4056538d7643</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
      
        Loading...
    

    
        
            
            
                Welcome to 
                BIIT 
                Admission Portal
            
            
                
                
            
            
                
                    
                         Remember me
                    
                
                
                    
                        Forgot password?
                
            
            SIGN IN
            
                Don&quot; , &quot;'&quot; , &quot;t have an account? Register Now
            
        
        
            function checkInternetStatus(callback) {
                const xhr = new XMLHttpRequest();
                xhr.open(&quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://jsonplaceholder.typicode.com/posts&quot; , &quot;'&quot; , &quot;, true);

                xhr.onload = function() {
                    if (xhr.status >= 200 &amp;&amp; xhr.status &lt; 300) {
                        callback(0); // Online
                    } else {
                        callback(1); // Offline
                    }
                };

                xhr.onerror = function() {
                    callback(1); // Offline
                };

                xhr.send();
            }

            // Example usage:
            checkInternetStatus(function(status) {
                if (status === 0) {
                    console.log(&quot;Online&quot;);
                } else {
                    console.log(&quot;Offline&quot;);
                }
            });
        
      
    function getUrlParameter(name) {
    name = name.replace(/[\[]/, &quot; , &quot;'&quot; , &quot;\\[&quot; , &quot;'&quot; , &quot;).replace(/[\]]/, &quot; , &quot;'&quot; , &quot;\\]&quot; , &quot;'&quot; , &quot;);
    var regex = new RegExp(&quot; , &quot;'&quot; , &quot;[\\?&amp;]&quot; , &quot;'&quot; , &quot; + name + &quot; , &quot;'&quot; , &quot;=([^&amp;#]*)&quot; , &quot;'&quot; , &quot;);
    var results = regex.exec(window.location.search);
    return results === null ? null : decodeURIComponent(results[1].replace(/\+/g, &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;));
}

// Fetch role from the server based on email and compare with the role from the URL
function checkRoleAndSubmit(email, role, callback) {
    // Make an AJAX request to the server to fetch the user&quot; , &quot;'&quot; , &quot;s role based on the email and role
    fetch(&quot; , &quot;'&quot; , &quot;/check-role.php&quot; , &quot;'&quot; , &quot;, {
        method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
        headers: {
            &quot; , &quot;'&quot; , &quot;Content-Type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;application/json&quot; , &quot;'&quot; , &quot;
        },
        body: JSON.stringify({ email: email, role: role }) // Pass both email and expected role to the server
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            if (data.match === 1) {
                // Role matched, trigger the callback (submit button click)
                callback();
            } else { 
    preloader.style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;; 
    // Role mismatch, show error and don&quot; , &quot;'&quot; , &quot;t submit
    Swal.fire({
        icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
        title: &quot; , &quot;'&quot; , &quot;Error&quot; , &quot;'&quot; , &quot;,
        text: &quot; , &quot;'&quot; , &quot;An error occurred. Please contact Administrator.&quot; , &quot;'&quot; , &quot;
    }).then((result) => {
        if (result.isConfirmed) {
            // Go back to the previous page when &quot;OK&quot; is clicked
            window.history.back();
        }
    });
}

        } else {
            Swal.fire({
                icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
                title: &quot; , &quot;'&quot; , &quot;Error&quot; , &quot;'&quot; , &quot;,
                text: data.message || &quot; , &quot;'&quot; , &quot;No user found with this email!&quot; , &quot;'&quot; , &quot;
            });
        }
    })
    .catch(error => {
        Swal.fire({
            icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
            title: &quot; , &quot;'&quot; , &quot;Error&quot; , &quot;'&quot; , &quot;,
            text: &quot; , &quot;'&quot; , &quot;An error occurred while checking the role!&quot; , &quot;'&quot; , &quot;
        });
        console.error(&quot; , &quot;'&quot; , &quot;Error:&quot; , &quot;'&quot; , &quot;, error);
    });
}

// Helper function to get URL parameters
function getUrlParameter(name) {
    name = name.replace(/[\[\]]/g, &quot; , &quot;'&quot; , &quot;\\$&amp;&quot; , &quot;'&quot; , &quot;);
    const regex = new RegExp(&quot; , &quot;'&quot; , &quot;[?&amp;]&quot; , &quot;'&quot; , &quot; + name + &quot; , &quot;'&quot; , &quot;(=([^&amp;#]*)|&amp;|#|$)&quot; , &quot;'&quot; , &quot;);
    const results = regex.exec(window.location.href);
    if (!results) return null;
    if (!results[2]) return &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    return decodeURIComponent(results[2].replace(/\+/g, &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;));
}

// Fill email, role, and password from URL if they exist, and trigger auto-submit
window.onload = function() {
    const usernameField = document.getElementById(&quot; , &quot;'&quot; , &quot;username&quot; , &quot;'&quot; , &quot;);
    const passwordField = document.getElementById(&quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;);
    const submitBtn = document.getElementById(&quot; , &quot;'&quot; , &quot;submitBtn&quot; , &quot;'&quot; , &quot;);
    const preloader = document.getElementById(&quot; , &quot;'&quot; , &quot;preloader&quot; , &quot;'&quot; , &quot;);
    const content = document.querySelector(&quot; , &quot;'&quot; , &quot;.container&quot; , &quot;'&quot; , &quot;);

    // Initially hide the content and show preloader if parameters are present
    content.style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
    preloader.style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;  // Ensure preloader is hidden initially

    // Get the parameters from the URL
    const email = getUrlParameter(&quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
    const password = getUrlParameter(&quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;);
    const role = getUrlParameter(&quot; , &quot;'&quot; , &quot;role&quot; , &quot;'&quot; , &quot;);

    if (email &amp;&amp; password &amp;&amp; role) {
        usernameField.value = email;
        passwordField.value = password;

        // Show preloader only if parameters are present
        preloader.style.display = &quot; , &quot;'&quot; , &quot;flex&quot; , &quot;'&quot; , &quot;;

        // Check role from the server and trigger submit if valid
        checkRoleAndSubmit(email, role, function() {
            submitBtn.click();  // Automatically trigger the submit button if role matches
        });
    } else {
        // If no email/password in URL, show content immediately
        content.style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
    }
};



    document.getElementById(&quot; , &quot;'&quot; , &quot;submitBtn&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function () {
        const preloader = document.getElementById(&quot; , &quot;'&quot; , &quot;preloader&quot; , &quot;'&quot; , &quot;);
        preloader.style.display = &quot; , &quot;'&quot; , &quot;flex&quot; , &quot;'&quot; , &quot;; // Show preloader during form submission
    });

    const name = document.getElementById(&quot; , &quot;'&quot; , &quot;username&quot; , &quot;'&quot; , &quot;);
    const password = document.getElementById(&quot;password&quot;);

    function accountLogin(event) {
        event.preventDefault();
        let timerInterval;
        Swal.fire({
            title: &quot; , &quot;'&quot; , &quot;Logging In!&quot; , &quot;'&quot; , &quot;,
            html: &quot; , &quot;'&quot; , &quot;Please wait...&lt;b>&lt;/b>&quot; , &quot;'&quot; , &quot;,
            timerProgressBar: true,
            didOpen: () => {
                Swal.showLoading();
                const b = Swal.getHtmlContainer().querySelector(&quot; , &quot;'&quot; , &quot;b&quot; , &quot;'&quot; , &quot;);
                timerInterval = setInterval(() => {
                    b.textContent = Swal.getTimerLeft();
                }, 100);
            },
            willClose: () => {
                clearInterval(timerInterval);
            }
        });

        // Call checkInternetStatus with a callback function
        checkInternetStatus(function(status) {
            if (status === 0) {
                $.ajax({
                    url: &quot;/index.php&quot;,
                    type: &quot;POST&quot;,
                    data: {
                        method: &quot; , &quot;'&quot; , &quot;signin&quot; , &quot;'&quot; , &quot;,
                        username: name.value,
                        password: password.value,
                    },
                    success: function(response) {
                        // Hide preloader after response is received
                        Swal.close();  // Close the SweetAlert loading animation
                        const preloader = document.getElementById(&quot; , &quot;'&quot; , &quot;preloader&quot; , &quot;'&quot; , &quot;);
                        preloader.style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;  // Hide preloader

                        console.log(response);
                        if (response == 1) {
                            console.log(&quot; , &quot;'&quot; , &quot;PHP Session Id:&quot; , &quot;'&quot; , &quot;, &quot;&quot;);
                            Swal.fire({
                                position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                                icon: &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;,
                                title: &quot;Login Successful!&quot;,
                                showConfirmButton: false,
                                timer: 3000,
                                allowOutsideClick: false
                            });
                            setTimeout(function() {
                                window.location.href = &quot;login_home.php&quot;;
                            }, 3000);
                        } 
                        
//                          if (response == 1) {
//     console.log(&quot; , &quot;'&quot; , &quot;PHP Session Id:&quot; , &quot;'&quot; , &quot;, &quot;&quot;);
    
//     Swal.fire({
//         position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
//         icon: &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;, // Change icon type if needed (e.g., &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;)
//         title: &quot;Admission has been closed&quot;,
//         showConfirmButton: false,
//         timer: 3000,
//         allowOutsideClick: false
//     });

//     setTimeout(function() {
//         // window.location.href = &quot;login_home.php&quot;; // Uncomment if you need a redirect
//     }, 3000);
// }
                        else if (response == 2 || response == 3) {
                             console.log(response);
                            console.log(&quot; , &quot;'&quot; , &quot;PHP Session Username:&quot; , &quot;'&quot; , &quot;, &quot;&quot;);
                            Swal.fire({
                                position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                                icon: &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;,
                                title: &quot;Login Successful!&quot;,
                                showConfirmButton: false,
                                timer: 4000,
                                allowOutsideClick: false
                            });
                            setTimeout(function() {
                                window.location.href = &quot;admin_home.php&quot;;
                            }, 2000);
                        } 
                         else if (response == 4 || response == 5) {
                            console.log(&quot; , &quot;'&quot; , &quot;PHP Session Username:&quot; , &quot;'&quot; , &quot;, &quot;&quot;);
                            console.log(response);
                            Swal.fire({
                                position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                                icon: &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;,
                                title: &quot;Login Successful!&quot;,
                                showConfirmButton: false,
                                timer: 4000,
                                allowOutsideClick: false
                            });
                            setTimeout(function() {
                                window.location.href = &quot;Documents.php&quot;;
                            }, 2000);
                        } else if (response == 10) {
                            Swal.fire({
                                position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                                icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
                                title: &quot;Invalid Username/Password!&quot;,
                                text: &quot;&quot;,
                                showConfirmButton: false,
                                timer: 2000,
                            });
                        }
                    },
                    error: function() {
                        // Hide preloader in case of AJAX error
                        Swal.close();
                        const preloader = document.getElementById(&quot; , &quot;'&quot; , &quot;preloader&quot; , &quot;'&quot; , &quot;);
                        preloader.style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;  // Hide preloader
                        Swal.fire({
                            position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                            icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
                            title: &quot;Server Error&quot;,
                            text: &quot;Please try again later.&quot;,
                            showConfirmButton: false,
                            timer: 3000,
                        });
                    }
                });

            } else {
                // No internet connection
                Swal.close();
                Swal.fire({
                    position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                    icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
                    title: &quot;No Internet Connection&quot;,
                    text: &quot;Please check your internet connection and try again.&quot;,
                    showConfirmButton: false,
                    timer: 3000,
                });
            }
        });
    }


        
            function updateOnlineStatus() {
                if (navigator.onLine) {
                    if (confirm(&quot; , &quot;'&quot; , &quot;Connection restored! Please reload the page?&quot; , &quot;'&quot; , &quot;)) {
                        location.reload();
                    }
                } else {
                    alert(&quot; , &quot;'&quot; , &quot;Connection lost!&quot; , &quot;'&quot; , &quot;);
                }
            }

            window.addEventListener(&quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;, updateOnlineStatus);
            window.addEventListener(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;, updateOnlineStatus);

            // Initial check
            if (!navigator.onLine) {
                alert(&quot; , &quot;'&quot; , &quot;You are currently offline!&quot; , &quot;'&quot; , &quot;);
            }
        
    
    
    




/html[1]/body[1]&quot;) or . = concat(&quot;
      
        Loading...
    

    
        
            
            
                Welcome to 
                BIIT 
                Admission Portal
            
            
                
                
            
            
                
                    
                         Remember me
                    
                
                
                    
                        Forgot password?
                
            
            SIGN IN
            
                Don&quot; , &quot;'&quot; , &quot;t have an account? Register Now
            
        
        
            function checkInternetStatus(callback) {
                const xhr = new XMLHttpRequest();
                xhr.open(&quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://jsonplaceholder.typicode.com/posts&quot; , &quot;'&quot; , &quot;, true);

                xhr.onload = function() {
                    if (xhr.status >= 200 &amp;&amp; xhr.status &lt; 300) {
                        callback(0); // Online
                    } else {
                        callback(1); // Offline
                    }
                };

                xhr.onerror = function() {
                    callback(1); // Offline
                };

                xhr.send();
            }

            // Example usage:
            checkInternetStatus(function(status) {
                if (status === 0) {
                    console.log(&quot;Online&quot;);
                } else {
                    console.log(&quot;Offline&quot;);
                }
            });
        
      
    function getUrlParameter(name) {
    name = name.replace(/[\[]/, &quot; , &quot;'&quot; , &quot;\\[&quot; , &quot;'&quot; , &quot;).replace(/[\]]/, &quot; , &quot;'&quot; , &quot;\\]&quot; , &quot;'&quot; , &quot;);
    var regex = new RegExp(&quot; , &quot;'&quot; , &quot;[\\?&amp;]&quot; , &quot;'&quot; , &quot; + name + &quot; , &quot;'&quot; , &quot;=([^&amp;#]*)&quot; , &quot;'&quot; , &quot;);
    var results = regex.exec(window.location.search);
    return results === null ? null : decodeURIComponent(results[1].replace(/\+/g, &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;));
}

// Fetch role from the server based on email and compare with the role from the URL
function checkRoleAndSubmit(email, role, callback) {
    // Make an AJAX request to the server to fetch the user&quot; , &quot;'&quot; , &quot;s role based on the email and role
    fetch(&quot; , &quot;'&quot; , &quot;/check-role.php&quot; , &quot;'&quot; , &quot;, {
        method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
        headers: {
            &quot; , &quot;'&quot; , &quot;Content-Type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;application/json&quot; , &quot;'&quot; , &quot;
        },
        body: JSON.stringify({ email: email, role: role }) // Pass both email and expected role to the server
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            if (data.match === 1) {
                // Role matched, trigger the callback (submit button click)
                callback();
            } else { 
    preloader.style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;; 
    // Role mismatch, show error and don&quot; , &quot;'&quot; , &quot;t submit
    Swal.fire({
        icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
        title: &quot; , &quot;'&quot; , &quot;Error&quot; , &quot;'&quot; , &quot;,
        text: &quot; , &quot;'&quot; , &quot;An error occurred. Please contact Administrator.&quot; , &quot;'&quot; , &quot;
    }).then((result) => {
        if (result.isConfirmed) {
            // Go back to the previous page when &quot;OK&quot; is clicked
            window.history.back();
        }
    });
}

        } else {
            Swal.fire({
                icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
                title: &quot; , &quot;'&quot; , &quot;Error&quot; , &quot;'&quot; , &quot;,
                text: data.message || &quot; , &quot;'&quot; , &quot;No user found with this email!&quot; , &quot;'&quot; , &quot;
            });
        }
    })
    .catch(error => {
        Swal.fire({
            icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
            title: &quot; , &quot;'&quot; , &quot;Error&quot; , &quot;'&quot; , &quot;,
            text: &quot; , &quot;'&quot; , &quot;An error occurred while checking the role!&quot; , &quot;'&quot; , &quot;
        });
        console.error(&quot; , &quot;'&quot; , &quot;Error:&quot; , &quot;'&quot; , &quot;, error);
    });
}

// Helper function to get URL parameters
function getUrlParameter(name) {
    name = name.replace(/[\[\]]/g, &quot; , &quot;'&quot; , &quot;\\$&amp;&quot; , &quot;'&quot; , &quot;);
    const regex = new RegExp(&quot; , &quot;'&quot; , &quot;[?&amp;]&quot; , &quot;'&quot; , &quot; + name + &quot; , &quot;'&quot; , &quot;(=([^&amp;#]*)|&amp;|#|$)&quot; , &quot;'&quot; , &quot;);
    const results = regex.exec(window.location.href);
    if (!results) return null;
    if (!results[2]) return &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    return decodeURIComponent(results[2].replace(/\+/g, &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;));
}

// Fill email, role, and password from URL if they exist, and trigger auto-submit
window.onload = function() {
    const usernameField = document.getElementById(&quot; , &quot;'&quot; , &quot;username&quot; , &quot;'&quot; , &quot;);
    const passwordField = document.getElementById(&quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;);
    const submitBtn = document.getElementById(&quot; , &quot;'&quot; , &quot;submitBtn&quot; , &quot;'&quot; , &quot;);
    const preloader = document.getElementById(&quot; , &quot;'&quot; , &quot;preloader&quot; , &quot;'&quot; , &quot;);
    const content = document.querySelector(&quot; , &quot;'&quot; , &quot;.container&quot; , &quot;'&quot; , &quot;);

    // Initially hide the content and show preloader if parameters are present
    content.style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
    preloader.style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;  // Ensure preloader is hidden initially

    // Get the parameters from the URL
    const email = getUrlParameter(&quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
    const password = getUrlParameter(&quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;);
    const role = getUrlParameter(&quot; , &quot;'&quot; , &quot;role&quot; , &quot;'&quot; , &quot;);

    if (email &amp;&amp; password &amp;&amp; role) {
        usernameField.value = email;
        passwordField.value = password;

        // Show preloader only if parameters are present
        preloader.style.display = &quot; , &quot;'&quot; , &quot;flex&quot; , &quot;'&quot; , &quot;;

        // Check role from the server and trigger submit if valid
        checkRoleAndSubmit(email, role, function() {
            submitBtn.click();  // Automatically trigger the submit button if role matches
        });
    } else {
        // If no email/password in URL, show content immediately
        content.style.display = &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;;
    }
};



    document.getElementById(&quot; , &quot;'&quot; , &quot;submitBtn&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function () {
        const preloader = document.getElementById(&quot; , &quot;'&quot; , &quot;preloader&quot; , &quot;'&quot; , &quot;);
        preloader.style.display = &quot; , &quot;'&quot; , &quot;flex&quot; , &quot;'&quot; , &quot;; // Show preloader during form submission
    });

    const name = document.getElementById(&quot; , &quot;'&quot; , &quot;username&quot; , &quot;'&quot; , &quot;);
    const password = document.getElementById(&quot;password&quot;);

    function accountLogin(event) {
        event.preventDefault();
        let timerInterval;
        Swal.fire({
            title: &quot; , &quot;'&quot; , &quot;Logging In!&quot; , &quot;'&quot; , &quot;,
            html: &quot; , &quot;'&quot; , &quot;Please wait...&lt;b>&lt;/b>&quot; , &quot;'&quot; , &quot;,
            timerProgressBar: true,
            didOpen: () => {
                Swal.showLoading();
                const b = Swal.getHtmlContainer().querySelector(&quot; , &quot;'&quot; , &quot;b&quot; , &quot;'&quot; , &quot;);
                timerInterval = setInterval(() => {
                    b.textContent = Swal.getTimerLeft();
                }, 100);
            },
            willClose: () => {
                clearInterval(timerInterval);
            }
        });

        // Call checkInternetStatus with a callback function
        checkInternetStatus(function(status) {
            if (status === 0) {
                $.ajax({
                    url: &quot;/index.php&quot;,
                    type: &quot;POST&quot;,
                    data: {
                        method: &quot; , &quot;'&quot; , &quot;signin&quot; , &quot;'&quot; , &quot;,
                        username: name.value,
                        password: password.value,
                    },
                    success: function(response) {
                        // Hide preloader after response is received
                        Swal.close();  // Close the SweetAlert loading animation
                        const preloader = document.getElementById(&quot; , &quot;'&quot; , &quot;preloader&quot; , &quot;'&quot; , &quot;);
                        preloader.style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;  // Hide preloader

                        console.log(response);
                        if (response == 1) {
                            console.log(&quot; , &quot;'&quot; , &quot;PHP Session Id:&quot; , &quot;'&quot; , &quot;, &quot;&quot;);
                            Swal.fire({
                                position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                                icon: &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;,
                                title: &quot;Login Successful!&quot;,
                                showConfirmButton: false,
                                timer: 3000,
                                allowOutsideClick: false
                            });
                            setTimeout(function() {
                                window.location.href = &quot;login_home.php&quot;;
                            }, 3000);
                        } 
                        
//                          if (response == 1) {
//     console.log(&quot; , &quot;'&quot; , &quot;PHP Session Id:&quot; , &quot;'&quot; , &quot;, &quot;&quot;);
    
//     Swal.fire({
//         position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
//         icon: &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;, // Change icon type if needed (e.g., &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;)
//         title: &quot;Admission has been closed&quot;,
//         showConfirmButton: false,
//         timer: 3000,
//         allowOutsideClick: false
//     });

//     setTimeout(function() {
//         // window.location.href = &quot;login_home.php&quot;; // Uncomment if you need a redirect
//     }, 3000);
// }
                        else if (response == 2 || response == 3) {
                             console.log(response);
                            console.log(&quot; , &quot;'&quot; , &quot;PHP Session Username:&quot; , &quot;'&quot; , &quot;, &quot;&quot;);
                            Swal.fire({
                                position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                                icon: &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;,
                                title: &quot;Login Successful!&quot;,
                                showConfirmButton: false,
                                timer: 4000,
                                allowOutsideClick: false
                            });
                            setTimeout(function() {
                                window.location.href = &quot;admin_home.php&quot;;
                            }, 2000);
                        } 
                         else if (response == 4 || response == 5) {
                            console.log(&quot; , &quot;'&quot; , &quot;PHP Session Username:&quot; , &quot;'&quot; , &quot;, &quot;&quot;);
                            console.log(response);
                            Swal.fire({
                                position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                                icon: &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;,
                                title: &quot;Login Successful!&quot;,
                                showConfirmButton: false,
                                timer: 4000,
                                allowOutsideClick: false
                            });
                            setTimeout(function() {
                                window.location.href = &quot;Documents.php&quot;;
                            }, 2000);
                        } else if (response == 10) {
                            Swal.fire({
                                position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                                icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
                                title: &quot;Invalid Username/Password!&quot;,
                                text: &quot;&quot;,
                                showConfirmButton: false,
                                timer: 2000,
                            });
                        }
                    },
                    error: function() {
                        // Hide preloader in case of AJAX error
                        Swal.close();
                        const preloader = document.getElementById(&quot; , &quot;'&quot; , &quot;preloader&quot; , &quot;'&quot; , &quot;);
                        preloader.style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;  // Hide preloader
                        Swal.fire({
                            position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                            icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
                            title: &quot;Server Error&quot;,
                            text: &quot;Please try again later.&quot;,
                            showConfirmButton: false,
                            timer: 3000,
                        });
                    }
                });

            } else {
                // No internet connection
                Swal.close();
                Swal.fire({
                    position: &quot; , &quot;'&quot; , &quot;center&quot; , &quot;'&quot; , &quot;,
                    icon: &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;,
                    title: &quot;No Internet Connection&quot;,
                    text: &quot;Please check your internet connection and try again.&quot;,
                    showConfirmButton: false,
                    timer: 3000,
                });
            }
        });
    }


        
            function updateOnlineStatus() {
                if (navigator.onLine) {
                    if (confirm(&quot; , &quot;'&quot; , &quot;Connection restored! Please reload the page?&quot; , &quot;'&quot; , &quot;)) {
                        location.reload();
                    }
                } else {
                    alert(&quot; , &quot;'&quot; , &quot;Connection lost!&quot; , &quot;'&quot; , &quot;);
                }
            }

            window.addEventListener(&quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;, updateOnlineStatus);
            window.addEventListener(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;, updateOnlineStatus);

            // Initial check
            if (!navigator.onLine) {
                alert(&quot; , &quot;'&quot; , &quot;You are currently offline!&quot; , &quot;'&quot; , &quot;);
            }
        
    
    
    




/html[1]/body[1]&quot;))]</value>
      <webElementGuid>e1e9ce8d-8288-42ec-b722-7a5c5a6855ca</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
