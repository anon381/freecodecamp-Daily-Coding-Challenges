
def get_laptop_cost(prices, budget):
    # Remove duplicates and keep only prices within budget
    valid_prices = sorted({p for p in prices if p <= budget}, reverse=True)
    
    if not valid_prices:
        return 0
    # Return second most expensive if exists, else the most expensive
    return valid_prices[1] if len(valid_prices) > 1 else valid_prices[0]
