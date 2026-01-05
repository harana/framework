def handle(event: dict) -> dict:
    username = event.get("username")
    print(f"Processing login for user: {username}")
    
    return {
        "success": True,
        "message": f"Login event processed for user: {username}"
    }
