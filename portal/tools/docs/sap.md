# SAP Tools

Provider: `sap` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the SAP S/4HANA OData API. They allow AI agents to manage suppliers, purchase orders, invoices, products, and sales orders in SAP. SAP S/4HANA is the leading enterprise ERP system used by large organizations for core business processes in finance, procurement, supply chain, and sales.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with SAP
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `BUSPARTNER`, `PURCHASE_ORDER`, `SUPPLIER_INVOICE`, `PRODUCT`, `SALES_ORDER`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sap_list_suppliers` | List SAP suppliers/vendors | GET | /sap/opu/odata/sap/API_BUSINESS_PARTNER/A_Supplier |
| `sap_get_supplier` | Get supplier details | GET | /sap/opu/odata/sap/API_BUSINESS_PARTNER/A_Supplier('{supplierId}') |
| `sap_list_purchase_orders` | List purchase orders | GET | /sap/opu/odata/sap/API_PURCHASEORDER_PROCESS_SRV/A_PurchaseOrder |
| `sap_get_purchase_order` | Get purchase order details | GET | /sap/opu/odata/sap/API_PURCHASEORDER_PROCESS_SRV/A_PurchaseOrder('{poNumber}') |
| `sap_create_purchase_order` | Create a purchase order | POST | /sap/opu/odata/sap/API_PURCHASEORDER_PROCESS_SRV/A_PurchaseOrder |
| `sap_list_invoices` | List invoices | GET | /sap/opu/odata/sap/API_SUPPLIERINVOICE_PROCESS_SRV/A_SupplierInvoice |
| `sap_get_invoice` | Get invoice details | GET | /sap/opu/odata/sap/API_SUPPLIERINVOICE_PROCESS_SRV/A_SupplierInvoice('{invoiceId}') |
| `sap_list_products` | List materials/products | GET | /sap/opu/odata/sap/API_PRODUCT_SRV/A_Product |
| `sap_get_product` | Get product details | GET | /sap/opu/odata/sap/API_PRODUCT_SRV/A_Product('{productId}') |
| `sap_list_sales_orders` | List sales orders | GET | /sap/opu/odata/sap/API_SALES_ORDER_SRV/A_SalesOrder |

---

## Tool Details

### sap_list_suppliers

**What it does**: Lists suppliers (vendors) in SAP S/4HANA. Filter by company code, purchasing organization, and supplier group.

**When to use**: Find existing suppliers, search for vendors by region, build a supplier list for procurement.

**Arguments**:
- `filter` (optional): OData filter expression
- `top` (optional): Max results (default 50)

**Example LLM prompt**: "List all suppliers in company code 1000"

---

### sap_get_supplier

**What it does**: Gets detailed information about a specific SAP supplier including addresses and contacts.

**When to use**: View complete supplier profile, check payment terms, see purchasing organization assignments.

**Arguments**:
- `supplierId` (required): Supplier ID

**Example LLM prompt**: "Get details for supplier L-00123"

---

### sap_list_purchase_orders

**What it does**: Lists purchase orders in SAP S/4HANA. Filter by status, supplier, purchasing organization, and date range.

**When to use**: Track purchase order status, find POs by supplier, monitor procurement pipeline.

**Arguments**:
- `supplierId` (optional): Filter by supplier ID
- `purchaseOrderStatus` (optional): PO status (I=incomplete, H=held, A=approved, E=error, P=pending)
- `fromDate` (optional): From date (YYYY-MM-DD)
- `toDate` (optional): To date (YYYY-MM-DD)
- `top` (optional): Max results (default 50)

**Example LLM prompt**: "List all approved purchase orders from January 2024"

---

### sap_get_purchase_order

**What it does**: Gets detailed information about a specific purchase order including items and schedule lines.

**When to use**: Check PO details, see line items and quantities, verify pricing and delivery dates.

**Arguments**:
- `poNumber` (required): Purchase order number

**Example LLM prompt**: "Get details for purchase order 4500001234"

---

### sap_create_purchase_order

**What it does**: Creates a new purchase order in SAP S/4HANA. Set supplier, items, quantities, and pricing.

**When to use**: Create new purchase orders, process procurement requests, order from existing suppliers.

**Arguments**:
- `supplierId` (required): Supplier ID
- `purchaseOrderType` (optional): PO type (e.g., NB=standard PO)
- `companyCode` (optional): Company code
- `purchasingOrganization` (optional): Purchasing organization
- `items` (optional): PO line items (array)

**Example LLM prompt**: "Create a purchase order for supplier L-00123 with standard PO type"

---

### sap_list_invoices

**What it does**: Lists invoices (incoming supplier invoices) in SAP S/4HANA. Filter by status, supplier, company code, and date range.

**When to use**: Find supplier invoices, track accounts payable, search by date or status.

**Arguments**:
- `supplierId` (optional): Filter by supplier ID
- `invoiceStatus` (optional): Invoice status
- `fromDate` (optional): From date (YYYY-MM-DD)
- `toDate` (optional): To date (YYYY-MM-DD)
- `top` (optional): Max results (default 50)

**Example LLM prompt**: "List all open invoices from supplier L-00123"

---

### sap_get_invoice

**What it does**: Gets detailed information about a specific supplier invoice including line items and payment terms.

**When to use**: View invoice details, check line items, verify billing amounts and payment status.

**Arguments**:
- `invoiceId` (required): Supplier invoice ID

**Example LLM prompt**: "Get details for invoice 5100001234"

---

### sap_list_products

**What it does**: Lists materials and products in SAP S/4HANA. Filter by type, group, plant, and storage location.

**When to use**: Browse product catalog, check product availability, find materials by group.

**Arguments**:
- `productType` (optional): Product type filter
- `productGroup` (optional): Product group
- `plant` (optional): Plant
- `top` (optional): Max results (default 50)

**Example LLM prompt**: "List all finished products in plant P001"

---

### sap_get_product

**What it does**: Gets detailed information about a specific product including descriptions, specifications, and warehouse data.

**When to use**: Check product pricing, view inventory levels, understand material specifications.

**Arguments**:
- `productId` (required): Product ID

**Example LLM prompt**: "Get details for product FG-10050"

---

### sap_list_sales_orders

**What it does**: Lists sales orders in SAP S/4HANA. Filter by status, customer, sales organization, and date range.

**When to use**: Track order status, find orders by customer, monitor sales pipeline.

**Arguments**:
- `customerId` (optional): Customer ID
- `salesOrderStatus` (optional): Sales order status
- `salesOrganization` (optional): Sales organization
- `fromDate` (optional): From date (YYYY-MM-DD)
- `toDate` (optional): To date (YYYY-MM-DD)
- `top` (optional): Max results (default 50)

**Example LLM prompt**: "List all open sales orders for customer C00123"

---

## SAP API Notes

- **OData API**: SAP S/4HANA uses OData protocol for REST API access
- **Business Partner**: Supplier and customer data is managed in the Business Partner framework
- **Purchase Order Number**: Format typically includes document type prefix and sequential number
- **Material Number**: Products are identified by material numbers (may be padded)
- **Supplier ID**: Suppliers have unique IDs, often prefixed with 'L-' for vendor accounts
- **Company Code**: SAP entity for legal accounting; controls currency and fiscal rules
- **PO Status Codes**: I=Incomplete, H=Held, A=Approved, E=Error, P=Pending approval
- **Sales Organizations**: Controls which sales areas a customer can be sold to
