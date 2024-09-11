package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.main.TestCaseMain


/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p></p>
     */
    public static Object host
     
    /**
     * <p></p>
     */
    public static Object $timestamp
     
    /**
     * <p></p>
     */
    public static Object password
     
    /**
     * <p></p>
     */
    public static Object email
     
    /**
     * <p></p>
     */
    public static Object refreshToken
     
    /**
     * <p></p>
     */
    public static Object accessToken
     
    /**
     * <p></p>
     */
    public static Object userId
     
    /**
     * <p></p>
     */
    public static Object unitId
     
    /**
     * <p></p>
     */
    public static Object categoryId
     
    /**
     * <p></p>
     */
    public static Object customerId
     
    /**
     * <p></p>
     */
    public static Object productId
     
    /**
     * <p></p>
     */
    public static Object officeId
     
    /**
     * <p></p>
     */
    public static Object currentdate
     
    /**
     * <p></p>
     */
    public static Object futuredate
     
    /**
     * <p></p>
     */
    public static Object saleId
     
    /**
     * <p></p>
     */
    public static Object purchaseId
     
    /**
     * <p></p>
     */
    public static Object name
     

    static {
        try {
            def selectedVariables = TestCaseMain.getGlobalVariables("default")
			selectedVariables += TestCaseMain.getGlobalVariables(RunConfiguration.getExecutionProfile())
            selectedVariables += TestCaseMain.getParsedValues(RunConfiguration.getOverridingParameters(), selectedVariables)
    
            host = selectedVariables['host']
            $timestamp = selectedVariables['$timestamp']
            password = selectedVariables['password']
            email = selectedVariables['email']
            refreshToken = selectedVariables['refreshToken']
            accessToken = selectedVariables['accessToken']
            userId = selectedVariables['userId']
            unitId = selectedVariables['unitId']
            categoryId = selectedVariables['categoryId']
            customerId = selectedVariables['customerId']
            productId = selectedVariables['productId']
            officeId = selectedVariables['officeId']
            currentdate = selectedVariables['currentdate']
            futuredate = selectedVariables['futuredate']
            saleId = selectedVariables['saleId']
            purchaseId = selectedVariables['purchaseId']
            name = selectedVariables['name']
            
        } catch (Exception e) {
            TestCaseMain.logGlobalVariableError(e)
        }
    }
}
